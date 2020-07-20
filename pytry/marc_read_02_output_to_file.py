"""
Usage:
- activate env
- cd to pytry directory
- $ python3 ./marc_read_02_output_to_file.py

This will create an output.txt file identical to the code in `exp07_to_file` (but slower!)
"""

import datetime, glob, os, pprint

from pymarc import MARCReader  # <https://gitlab.com/pymarc/pymarc>


SOURCE_FILES_DIR = './source_files'
OUTPUT_FILE = './output.txt'


def process_marc_files():

    first_start_time = datetime.datetime.now()

    ## clear out and create output file
    with open( OUTPUT_FILE, 'w' ) as fh:  # will clear any existing file
        pass

    ## get list of files
    marc_file_list: list = create_marc_file_list()

    ## get the appender file-handler
    with open( OUTPUT_FILE, 'a' ) as fh_a:  # will clear any existing file

        ## for each file...
        for marc_path in marc_file_list:

            file_start_time = datetime.datetime.now()
            # print( '\nnew file...' )
            # print( f'marc_file_path, ``{marc_path}``' )

            with open( marc_path, 'rb' ) as fh:
                reader = MARCReader( fh )
                for record in reader:
                    # print( '\nnew record...' )

                    # print( f'full_title, ``{record.title()}``'  )
                    fh_a.write( f'{record.title()}\n' )

                    bib = record['907']['a']
                    # print( f'bib_url, ``https://search.library.brown.edu/catalog/{bib[1:-1]}``' )
                    fh_a.write( f'https://search.library.brown.edu/catalog/{bib[1:-1]}\n\n' )

            file_end_time = datetime.datetime.now()
            print( f'\nfile-elapsed-time, ``{file_end_time - file_start_time}``' )

    all_files_end_time = datetime.datetime.now()
    all_files_elapsed = all_files_end_time - first_start_time  # yields type: <class 'datetime.timedelta'>
    minutes = all_files_elapsed.seconds / 60
    print( f'\nall-files-elapsed-time, ``{minutes}`` minutes\n' )

    ## end process_marc_files()


def create_marc_file_list():
    marc_file_list = sorted( glob.glob('%s/*.mrc' % SOURCE_FILES_DIR) )
    print( f'marc_file_list, ``{pprint.pformat(marc_file_list)}``' )
    return marc_file_list


if __name__ == '__main__':
    print( '\n-------\nstarting `main`' )
    process_marc_files()
    print( '`main` complete\n-------\n' )
