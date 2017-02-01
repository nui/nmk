#!/usr/bin/env python
import json
import logging
import os
import subprocess

from tempfile import NamedTemporaryFile
from six.moves.urllib import request

logging.basicConfig(level=logging.INFO)


def get_latest_release():
    api_url = 'https://api.github.com/repos/nuimk/nmk/releases/latest'
    rq = request.urlopen(api_url)
    return json.loads(rq.read())


def get_bundle_url(api_data):
    assets = api_data['assets']
    items = (x for x in assets if x['name'] == 'nmk.tar.gz')
    return next(items)['browser_download_url']


def download_bundle(url):
    rq = request.urlopen(url)
    data = rq.read()
    f = NamedTemporaryFile(suffix='.tar.gz')
    f.write(data)
    f.flush()
    logging.info('Wrote bundle data to ' + f.name)
    return f

api_response = get_latest_release()
logging.info('Found tag ' + api_response['tag_name'])

bundle_url = get_bundle_url(api_response)
logging.info('Downloading ' + bundle_url)
bundle_file = download_bundle(bundle_url)

nmk_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
os.chdir(nmk_dir)

logging.info('call sh uninstall.sh')
subprocess.call(['sh', 'uninstall.sh'])

logging.info('extract bundle')
subprocess.call(['tar', '-xf', bundle_file.name, '--strip-components=1'])

logging.info('Done')
