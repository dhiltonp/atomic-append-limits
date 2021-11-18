#!/bin/env python3

import argparse
import random
import string

from collections import defaultdict
from multiprocessing import Pool
from tempfile import NamedTemporaryFile
from time import sleep

def gen_message(length):
    s = ""
    for i in range(length-1):
        s += random.choice(string.ascii_lowercase)
    s += "\n"
    return s


def repeat_lines(id_, path, length, times):
    message = gen_message(length)
    with open(path, "a") as f:
        print(f"{id_} starting work")
        for i in range(times):
            f.write(message)
        print(f"{id_} finishing work")


def summarize(path, processes, times):
    print("summarizing results")
    error = False
    match_counts = defaultdict(int)
    with open(path) as g:
        for line in g.readlines():
            match_counts[line] += 1
    if len(match_counts) != processes:
        error = True
        print("ERROR: MISMATCH OF PROCESSES AND LINE TYPES")
        print(f"line types: {len(match_counts)}")
        print(f"processes: {processes}")
    
    lines_per_type = [match_count for match_count in match_counts.values() if match_count != times]
    if lines_per_type:
        error = True
        print("ERROR: MISMATCH OF LINES WRITTEN PER TYPE")
        print(lines_per_type)

    if error:
        print("errors found, sleeping 24h to allow manual inspection if desired")
        print(f"path: {f.name}")
        sleep(24*60*60)


def start_parallel_writes(path, processes, length, times):
    print(f"path: {path}")
    bytes_ = processes * length * times / 1024 / 1024
    print(f"writing {bytes_}MB in {length}B chunks")
    with Pool(processes) as p:
        p.starmap(repeat_lines, [(x, path, length, times) for x in range(processes)])
        p.close()
        p.join()
    print("writing complete!")


def start_thread_writes(path, threads, length, times):
    print(f"path: {path}")
    bytes_ = threads * length * times / 1024 / 1024
    print(f"writing {bytes_}MB in {length}B chunks (threads)")
    with ThreadPool(threads) as p:
        p.starmap(repeat_lines, [(x, path, length, times) for x in range(threads)])
        p.close()
        p.join()
    print("writing complete!")


def truncate(path):
    with open(path, 'w') as f:
        f.write('')




if __name__ == '__main__':
    parser = argparse.ArgumentParser('Attempt to induce file mangling when writing in append mode\n')
    parser.add_argument('-file', type=str, default=NamedTemporaryFile().name, help='file that will be truncated and reused as a write target')
    parser.add_argument('-processes', type=int, default=20, help='number of processes to write')
    parser.add_argument('-times', type=int, default=5000, help='number of times each process will write')
    parser.add_argument('-length', type=int, default=4096, help='length of each write')
    processes = 100
    times = 500
    length = 4096 * 9
    args = parser.parse_args()
    
    bytes_ = args.processes * args.length * args.times / 1024 / 1024
    print(f"writing {bytes_}MB in {args.length}B chunks across {args.processes} processes to '{args.file}'")
    print(args)
    sleep(4)
    
    for i in range(100000):
        print(f"starting run #{i}")
        truncate(args.file)
        start_parallel_writes(args.file, args.processes, args.length, args.times)
        summarize(args.file, args.processes, args.times)

    truncate(args.file)


