import datetime, glob, os, pprint

from pymarc import MARCReader  # <https://gitlab.com/pymarc/pymarc>


FILES_DIR = './source_files'


def process_marc_files():

    ## get list of files
    marc_file_list: list = create_marc_file_list()

    ## for each file...

        ## assemble full-title & print it

        ## assemble bib-url & print it

    ## end process_marc_files()


def create_marc_file_list():
    marc_file_list = sorted( glob.glob('%s/*.mrc' % FILES_DIR) )
    print( f'marc_file_list, ``{pprint.pformat(marc_file_list)}``' )
    return marc_file_list



def validate_marc_files():
    """ Checks downloaded marc files. """
    marc_file_list = sorted( glob.glob('%s/*.mrc' % FILE_DOWNLOAD_DIR) )
    print( f'marc_file_list, ``{marc_file_list}``' )
    start = datetime.datetime.now()
    for file_path in marc_file_list:
        print( f'file_path, ``{file_path}``' )
        size_in_mb = os.path.getsize( file_path ) / 1024 / 1024
        print( f'size, ``{size_in_mb}MB``' )
        validity = open_and_check_file( file_path )
        print( f'validity, ``{validity}``' )
    time_taken = str( datetime.datetime.now() - start )
    print( f'time_taken, ``{time_taken}``' )
    return

def open_and_check_file( file_path ):
    """ Opens file and ensures it can be read as marc.
        Called by validate_marc_files() """
    validity = False
    titles = []
    print( f'file about to be checked, ``{file_path}``' )
    with open( file_path, 'rb' ) as fh:
        reader = MARCReader( fh )
        try:
            for record in reader:
                titles.append( record.title() )
            print( f'titles, ``{pprint.pformat(titles)}``' )
            validity = True
        except Exception as e:
            print( f'problem reading file, ``{e}``' )
    return validity


if __name__ == '__main__':
    print( '\n-------\nstarting `main`' )
    process_marc_files()
    print( '`main` complete\n-------\n' )
