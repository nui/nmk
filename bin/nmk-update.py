#!/usr/bin/env python
from os import path
from tempfile import NamedTemporaryFile
import json
import logging
import os
import subprocess

from six.moves.urllib import request
import argparse

logging.basicConfig(format='{0}: %(message)s'.format(path.basename(__file__)),
                    level=logging.INFO)

NMK_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RELEASE_JSON_PATH = path.join(NMK_DIR, '.release.json')


def build_parser():
    parser = argparse.ArgumentParser()
    parser.add_argument('-s', '--stable',
                        dest='stable',
                        action='store_true',
                        default=False,
                        help='Download stable release from github')
    parser.add_argument('tag_name',
                        nargs=argparse.OPTIONAL,
                        help='git tag')
    return parser


def get_latest_release():
    response = request.urlopen('https://api.github.com/repos/nuimk/nmk/releases/latest')
    return json.loads(response.read())


def get_release(tag_name):
    response = request.urlopen('https://api.github.com/repos/nuimk/nmk/releases')
    releases = json.loads(response.read())
    return next((x for x in releases if x['tag_name'] == tag_name))


def get_bundle_info(release_info):
    assets = release_info['assets']
    return next((x for x in assets if x['name'] == 'nmk.tar.gz'))


def get_bundle_url(release_info):
    return get_bundle_info(release_info)['browser_download_url']


def download_bundle(url):
    rq = request.urlopen(url)
    tf = NamedTemporaryFile(suffix='.tar.gz')
    tf.write(rq.read())
    tf.flush()
    logging.info('Downloaded bundle data to ' + tf.name)
    return tf


def is_up2date(release_info):
    if not path.exists(RELEASE_JSON_PATH):
        logging.info('Not found saved release data')
        return False
    bundle_info = get_bundle_info(release_info)
    with open(RELEASE_JSON_PATH) as f:
        saved_release_info = json.loads(f.read())
        # 1. check the tag name
        if release_info['tag_name'] != saved_release_info['tag_name']:
            return False
        saved_bundle_info = get_bundle_info(saved_release_info)
        # 2. check bundle file metadata
        return all((saved_bundle_info[k] == bundle_info[k] for k in ('created_at', 'updated_at', 'size')))


def save_github_release_info(release_info):
    with open(RELEASE_JSON_PATH, 'w') as f:
        f.write(json.dumps(release_info, sort_keys=True, indent=4))
        f.flush()
        logging.info('Saved release json to {0}'.format(RELEASE_JSON_PATH))


def remove_github_release_info():
    if os.path.exists(RELEASE_JSON_PATH):
        os.remove(RELEASE_JSON_PATH)


def download_and_install(bundle_url):
    logging.info('Downloading ' + bundle_url)
    bundle_file = download_bundle(bundle_url)

    os.chdir(NMK_DIR)
    logging.info('Uninstall current version')
    subprocess.call(['sh', 'uninstall.sh'])
    logging.info('Extracting bundle')
    subprocess.call(['tar', '-xzf', bundle_file.name, '--strip-components=1'])
    bundle_file.close()


def main():
    args = build_parser().parse_args()

    if args.stable:
        tag_name = args.tag_name
        release_info = get_release(tag_name) if tag_name else get_latest_release()
        logging.info('Founded tag ' + release_info['tag_name'])
        if is_up2date(release_info):
            logging.info('Already up to date :)')
            exit(0)
        bundle_url = get_bundle_url(release_info)
        download_and_install(bundle_url)
        save_github_release_info(release_info)
    else:
        bundle_url = 'https://storage.googleapis.com/nuimk-nmk/nmk.tar.gz'
        download_and_install(bundle_url)
        remove_github_release_info()
    logging.info('Done')


if __name__ == '__main__':
    main()
