#!/usr/bin/env python
import json
import logging
import os
from os import path
import subprocess

from tempfile import NamedTemporaryFile
from six.moves.urllib import request

logging.basicConfig(level=logging.INFO)

NMK_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RELEASE_INFO_PATH = path.join(NMK_DIR, '.nmk-release')


def get_latest_release():
    api_url = 'https://api.github.com/repos/nuimk/nmk/releases/latest'
    rq = request.urlopen(api_url)
    return json.loads(rq.read())


def get_bundle_info(release_info):
    assets = release_info['assets']
    items = (x for x in assets if x['name'] == 'nmk.tar.gz')
    return next(items)


def get_bundle_url(release_info):
    return get_bundle_info(release_info)['browser_download_url']


def download_bundle(url):
    rq = request.urlopen(url)
    data = rq.read()
    f = NamedTemporaryFile(suffix='.tar.gz')
    f.write(data)
    f.flush()
    logging.info('Wrote bundle data to ' + f.name)
    return f


def is_up2date(release_info):
    try:
        bundle_info = get_bundle_info(release_info)
        with open(RELEASE_INFO_PATH) as f:
            saved_bundle_info = get_bundle_info(json.loads(f.read()))
            return all((saved_bundle_info[x] == bundle_info[x] for x in ['created_at', 'size']))
    except IOError:
        return False


def save_release_info(release_info):
    with open(RELEASE_INFO_PATH, 'w') as f:
        f.write(json.dumps(release_info))
        logging.info('Saved release info')


def main():
    release_info = get_latest_release()
    logging.info('Founded tag ' + release_info['tag_name'])
    if is_up2date(release_info):
        logging.info('Already up to date :)')
        exit(0)

    bundle_url = get_bundle_url(release_info)
    logging.info('Downloading ' + bundle_url)
    bundle_file = download_bundle(bundle_url)

    os.chdir(NMK_DIR)

    logging.info('Calling sh uninstall.sh')
    subprocess.call(['sh', 'uninstall.sh'])

    logging.info('Extracting bundle')
    subprocess.call(['tar', '-xf', bundle_file.name, '--strip-components=1'])

    save_release_info(release_info)
    logging.info('Done')

if __name__ == '__main__':
    main()
