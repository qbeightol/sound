#! /usr/bin/env python3

'''
A script that prints the contents of a file by printing each byte as both an 
number and a character.
'''

import os
import sys

def script():
    target = sys.argv[1] # what we should decode
    with open(target, 'r', encoding='ISO-8859-1') as target_file: 
        for byte in target_file.read():
            print(str(ord(byte)) + "\t(" + byte + ")");

if __name__ == '__main__':
    script()