"""
Run on Python 2.6.6 and later
"""

import gzip
import json
import logging
import platform
import os
import subprocess
import time
from abc import ABCMeta, abstractmethod, abstractproperty
from os import path
from tempfile import NamedTemporaryFile

from six.moves.urllib import request

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


def download_launcher(url):
    rq = request.urlopen(url)
    tf = NamedTemporaryFile()
    start = time.time()
    tf.write(rq.read())
    end = time.time()
    tf.flush()
    logging.debug('Downloaded in {0:.2f} s'.format(end - start))
    logging.debug('Downloaded data to ' + tf.name)
    return tf


def install_launcher(archive_file):
    os.chdir(NMK_DIR)
    launcher_path = path.join(NMK_DIR, 'bin', 'nmk')
    with open(launcher_path, 'wb') as f:
        archive_file.seek(0)
        f.write(gzip.open(archive_file.name, 'rb').read())
        f.close()
        subprocess.call(['chmod', '+x', launcher_path])
        logging.info('updated nmk')


def download_and_install_launcher(archive_url):
    logging.info('Downloading ' + archive_url)
    archive_file = download_launcher(archive_url)

    install_launcher(archive_file)
    archive_file.close()


def install(archive_file):
    os.chdir(NMK_DIR)
    logging.debug('Uninstalling')
    subprocess.call(['sh', 'uninstall.sh'])
    logging.debug('Extracting update data')
    subprocess.call(['tar', '-xf', archive_file.name, '--strip-components=1'])


def build_parser():
    parser = argparse.ArgumentParser()
    parser.add_argument('-d', '--debug',
                        dest='debug',
                        action='store_true',
                        default=False,
                        help='print debug message')
    return parser


def select_launcher():
    machine = platform.machine()
    part = 'armv7-linux' if machine == 'armv7l' else 'amd64-linux-musl'
    return 'https://storage.googleapis.com/nmk.nuimk.com/nmk.rs/nmk-' + part + '.gz'



def update_from_remote(args):
    resource = GoogleCloudStorageResource()

    resource.fetch()
    if resource.is_up2date():
        logging.info('Already up to date :)')
        exit(0)
    download_url = resource.download_url
    download_and_install(download_url)
    resource.save_to_cache()
    logging.info('Done')
    download_and_install_launcher(select_launcher())


def prevent_run_on_git():
    if path.isdir(path.join(NMK_DIR, '.git')):
        logging.error('Found .git dir')
        exit(1)


def main():
    args = build_parser().parse_args()
    if args.debug:
        logging.getLogger().setLevel(logging.DEBUG)

    prevent_run_on_git()
    update_from_remote(args)


if __name__ == '__main__':
    main()
