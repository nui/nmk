#!/usr/bin/env python
from abc import ABCMeta, abstractmethod, abstractproperty
from os import path
from tempfile import NamedTemporaryFile
import json
import logging
import os
import subprocess
import time

from six.moves.urllib import request
import argparse

logging.basicConfig(format='{0}: %(message)s'.format(path.basename(__file__)),
                    level=logging.INFO)

NMK_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


class ArchiveResource(object):
    __metaclass__ = ABCMeta

    @abstractproperty
    def cache_path(self):
        return NotImplemented

    @abstractproperty
    def download_url(self):
        return NotImplemented

    @abstractmethod
    def fetch(self):
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

    def fetch(self):
        response = request.urlopen('https://api.github.com/repos/nuimk/nmk/releases/latest')
        self.release = json.loads(response.read())

    def fetch_tag(self, tag_name):
        response = request.urlopen('https://api.github.com/repos/nuimk/nmk/releases')
        releases = json.loads(response.read())
        self.release = next((x for x in releases if x['tag_name'] == tag_name))

    @staticmethod
    def get_archive_info(release):
        assets = release['assets']
        return next((x for x in assets if x['name'] == 'nmk.tar.gz'))

    def is_up2date(self):
        if not path.exists(self.cache_path):
            logging.info('Not found Github API cache')
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
            logging.info('Cached Github release json in {0}'.format(self.cache_path))

    def clear_cache(self):
        if super(GithubReleaseResource, self).clear_cache():
            logging.info('Cleared Github API cache')


class GoogleCloudStorageResource(ArchiveResource):
    def __init__(self):
        self.resource = None

    @property
    def cache_path(self):
        return path.join(NMK_DIR, '.gcs.resource.json')

    @property
    def download_url(self):
        return self.resource['mediaLink']

    def fetch(self):
        response = request.urlopen('https://www.googleapis.com/storage/v1/b/nuimk-nmk/o/nmk.tar.gz')
        self.resource = json.loads(response.read())

    def is_up2date(self):
        if not path.exists(self.cache_path):
            logging.info('Not found GoogleCloudStorage API cache')
            return False
        with open(self.cache_path) as f:
            cached_resource = json.loads(f.read())
            return self.resource['md5Hash'] == cached_resource['md5Hash']

    def save_to_cache(self):
        with open(self.cache_path, 'w') as f:
            f.write(json.dumps(self.resource, sort_keys=True, indent=4))
            f.flush()
            logging.info('Cached GoogleCloudStorage object resource json in {0}'.format(self.cache_path))

    def clear_cache(self):
        if super(GoogleCloudStorageResource, self).clear_cache():
            logging.info('Cleared GoogleCloudStorage API cache')


def download_bundle(url):
    rq = request.urlopen(url)
    tf = NamedTemporaryFile(suffix='.tar.gz')
    start = time.time()
    tf.write(rq.read())
    end = time.time()
    tf.flush()
    logging.info('Downloaded in {0:.2f} s'.format(end - start))
    logging.info('Downloaded data to ' + tf.name)
    return tf


def download_and_install(archive_url):
    logging.info('Downloading ' + archive_url)
    archive_file = download_bundle(archive_url)

    os.chdir(NMK_DIR)
    logging.info('Uninstalling')
    subprocess.call(['sh', 'uninstall.sh'])
    logging.info('Extracting update data')
    subprocess.call(['tar', '-xzf', archive_file.name, '--strip-components=1'])
    archive_file.close()


def build_parser():
    parser = argparse.ArgumentParser()
    parser.add_argument('-s', '--stable',
                        dest='stable',
                        action='store_true',
                        default=False,
                        help='Update with stable release from Github')
    parser.add_argument('tag_name',
                        nargs=argparse.OPTIONAL,
                        help='git tag')
    return parser


def main():
    args = build_parser().parse_args()

    github = GithubReleaseResource()
    gcloud = GoogleCloudStorageResource()
    if args.stable:
        resource = github
        other_resource = gcloud
        tag_name = args.tag_name
        github.fetch_tag(tag_name) if tag_name else github.fetch()
        logging.info('Founded Github tag ' + github.tag_name)
    else:
        resource = gcloud
        other_resource = github
        resource.fetch()
    if resource.is_up2date():
        logging.info('Already up to date :)')
        exit(0)
    download_url = resource.download_url
    download_and_install(download_url)
    resource.save_to_cache()
    other_resource.clear_cache()
    logging.info('Done')


if __name__ == '__main__':
    main()
