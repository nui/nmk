#!/usr/bin/env python
"""
Run on Python 2.6.6 and later
"""
from abc import ABCMeta, abstractmethod, abstractproperty
from os import path
from tempfile import NamedTemporaryFile
import json
import logging
import os
import subprocess
import sys
import time

from six.moves.urllib import request
from six.moves import filter, input
from six import print_
import argparse
import six

logging.basicConfig(format='%(levelname)5s: %(message)s',
                    level=logging.INFO)

NMK_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def loads_json_api(uri):
    response = request.urlopen(uri)
    data = response.read()
    json_string = data.decode('utf-8') if isinstance(data, six.binary_type) else data
    return json.loads(json_string)


@six.add_metaclass(ABCMeta)
class ArchiveResource(object):
    @abstractproperty
    def cache_path(self):
        return NotImplemented

    @abstractproperty
    def download_url(self):
        return NotImplemented

    @abstractmethod
    def fetch(self, latest):
        return NotImplemented

    @abstractmethod
    def is_up2date(self):
        return NotImplemented

    @abstractmethod
    def save_to_cache(self):
        return NotImplemented

    @abstractmethod
    def clear_cache(self):
        if os.path.exists(self.cache_path):
            os.remove(self.cache_path)
            return True
        return False


class GithubReleaseResource(ArchiveResource):
    def __init__(self):
        self.release = None

    @property
    def cache_path(self):
        return path.join(NMK_DIR, '.github.release.json')

    @property
    def download_url(self):
        return self.get_archive_info(self.release)['browser_download_url']

    @property
    def tag_name(self):
        return self.release['tag_name']

    def fetch(self, latest):
        releases = loads_json_api('https://api.github.com/repos/nuimk/nmk/releases')
        releases = list(filter(self.has_archive, releases))
        if len(releases) == 0:
            logging.error('Not found updatable github release')
            sys.exit(1)
        self.release = releases[0] if latest else self.interactive_choose_release(releases)

    @staticmethod
    def interactive_choose_release(releases):
        print_('Select Github release to update\n')
        d = dict((k, v) for k, v in enumerate(releases, start=1))
        for i, release in six.iteritems(d):
            print_("  {0}) {1}".format(i, release['tag_name']))
        ch = input('\nEnter number of release (default to 1): ') or '1'
        release = d.get(int(ch))
        logging.debug('Selected {0}'.format(release['tag_name']))
        return release

    @staticmethod
    def has_archive(release):
        assets = release['assets']
        return any((x['name'] == 'nmk.tar.gz' for x in assets))

    @staticmethod
    def get_archive_info(release):
        assets = release['assets']
        return next((x for x in assets if x['name'] == 'nmk.tar.gz'))

    def is_up2date(self):
        if not path.exists(self.cache_path):
            logging.debug('Not found Github API cache')
            return False
        archive_info = self.get_archive_info(self.release)
        with open(self.cache_path) as f:
            cached_release = json.loads(f.read())
            # 1. check the tag name
            if self.release['tag_name'] != cached_release['tag_name']:
                return False
            cached_archive_info = self.get_archive_info(cached_release)
            # 2. check archive file metadata
            return all((cached_archive_info[k] == archive_info[k] for k in ('created_at', 'updated_at', 'size')))

    def save_to_cache(self):
        with open(self.cache_path, 'w') as f:
            f.write(json.dumps(self.release, sort_keys=True, indent=4))
            f.flush()
            logging.debug('Cached Github release json in {0}'.format(self.cache_path))

    def clear_cache(self):
        if super(GithubReleaseResource, self).clear_cache():
            logging.debug('Cleared Github API cache')


class GoogleCloudStorageResource(ArchiveResource):
    def __init__(self):
        self.resource = None

    @property
    def cache_path(self):
        return path.join(NMK_DIR, '.gcs.resource.json')

    @property
    def download_url(self):
        return self.resource['mediaLink']

    def fetch(self, latest):
        self.resource = loads_json_api('https://www.googleapis.com/storage/v1/b/nmk.nuimk.com/o/nmk.tar.gz')

    def is_up2date(self):
        if not path.exists(self.cache_path):
            logging.debug('Not found GoogleCloudStorage API cache')
            return False
        with open(self.cache_path) as f:
            cached_resource = json.loads(f.read())
            return self.resource['md5Hash'] == cached_resource['md5Hash']

    def save_to_cache(self):
        with open(self.cache_path, 'w') as f:
            f.write(json.dumps(self.resource, sort_keys=True, indent=4))
            f.flush()
            logging.debug('Cached GoogleCloudStorage object resource json in {0}'.format(self.cache_path))

    def clear_cache(self):
        if super(GoogleCloudStorageResource, self).clear_cache():
            logging.debug('Cleared GoogleCloudStorage API cache')


def download_bundle(url):
    rq = request.urlopen(url)
    tf = NamedTemporaryFile(suffix='.tar.gz')
    start = time.time()
    tf.write(rq.read())
    end = time.time()
    tf.flush()
    logging.debug('Downloaded in {0:.2f} s'.format(end - start))
    logging.debug('Downloaded data to ' + tf.name)
    return tf


def download_and_install(archive_url):
    logging.info('Downloading ' + archive_url)
    archive_file = download_bundle(archive_url)

    install(archive_file)
    archive_file.close()


def install(archive_file):
    os.chdir(NMK_DIR)
    logging.debug('Uninstalling')
    subprocess.call(['sh', 'uninstall.sh'])
    logging.debug('Extracting update data')
    subprocess.call(['tar', '-xf', archive_file.name, '--strip-components=1'])


def build_parser():
    parser = argparse.ArgumentParser()
    parser.add_argument('-s', '--stable',
                        dest='stable',
                        action='store_true',
                        default=False,
                        help='update using stable release from Github')
    parser.add_argument('-d', '--debug',
                        dest='debug',
                        action='store_true',
                        default=False,
                        help='print debug message')
    parser.add_argument('-f',
                        dest='input',
                        help='update using local file')
    parser.add_argument('-i',
                        dest='interactive',
                        action='store_true',
                        default=False,
                        help='interactive choose release')
    return parser


def update_from_file(args):
    with open(os.path.abspath(args.input), 'rb') as archive_file:
        install(archive_file)


def update_from_remote(args):
    github = GithubReleaseResource()
    gcloud = GoogleCloudStorageResource()
    resource = github if args.stable else gcloud
    other_resource = gcloud if args.stable else github

    resource.fetch(latest=not args.interactive)
    if resource.is_up2date():
        logging.info('Already up to date :)')
        exit(0)
    download_url = resource.download_url
    download_and_install(download_url)
    resource.save_to_cache()
    other_resource.clear_cache()
    logging.info('Done')


def main():
    args = build_parser().parse_args()
    if args.debug:
        logging.getLogger().setLevel(logging.DEBUG)

    if args.input:
        update_from_file(args)
    else:
        update_from_remote(args)


if __name__ == '__main__':
    main()
