import datetime, glob, os, pprint

from pymarc import MARCReader  # <https://gitlab.com/pymarc/pymarc>


FILES_DIR = './source_files'


def process_marc_files():

    file_start_time = datetime.datetime.now()

    ## get list of files
    marc_file_list: list = create_marc_file_list()

    ## for each file...
    for marc_path in marc_file_list:

        file_start_time = datetime.datetime.now()
        print( '\nnew file...' )
        print( f'marc_file_path, ``{marc_path}``' )

        with open( marc_path, 'rb' ) as fh:
            reader = MARCReader( fh )
            for record in reader:
                print( '\nnew record...' )

                print( f'full_title, ``{record.title()}``'  )

                bib = record['907']['a']
                print( f'bib_url, ``https://search.library.brown.edu/catalog/{bib[1:-1]}``' )

        file_end_time = datetime.datetime.now()
        print( f'\nfile-elapsed-time, ``{file_end_time - file_start_time}``' )

    all_files_end_time = datetime.datetime.now()
    print( f'\nall-files-elapsed-time, ``{all_files_end_time - file_start_time}``\n' )

    ## end process_marc_files()


def create_marc_file_list():
    marc_file_list = sorted( glob.glob('%s/*.mrc' % FILES_DIR) )
    print( f'marc_file_list, ``{pprint.pformat(marc_file_list)}``' )
    return marc_file_list


if __name__ == '__main__':
    print( '\n-------\nstarting `main`' )
    process_marc_files()
    print( '`main` complete\n-------\n' )
