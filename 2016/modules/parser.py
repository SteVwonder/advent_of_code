import argparse

def default_parser():
    parent_parser = argparse.ArgumentParser(add_help=False)
    parent_parser.add_argument('input_file')
    return parent_parser
