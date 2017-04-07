#!/usr/bin/python3
import os
import re
import subprocess


def edit_rc_file():
    config_file = os.path.expanduser('~/.config/plasma-org.kde.plasma.desktop-appletsrc')
    lines = open(config_file).read().splitlines()
    found_containment_1 = False
    for index, line in enumerate(lines):
        if not found_containment_1:
            if line.strip() == '[Containments][1]':
                found_containment_1 = True
        else:
            if re.match('^lastScreen=', line):
                lines[index] = 'lastScreen=0'
                break
    output = os.linesep.join(lines)
    open(config_file, 'w').write(output)

def restart_plasmashell():
    devnull = open(os.devnull, 'w')
    subprocess.run(['killall', 'plasmashell'], stdout=devnull, stderr=devnull)
    subprocess.run(['kstart', 'plasmashell'], stdout=devnull, stderr=devnull)

if __name__ == '__main__':
    edit_rc_file()
    restart_plasmashell()
