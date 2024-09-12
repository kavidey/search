# Imports
import os
import time
import mimetypes
import datetime
from pathlib import Path
from tqdm.auto import tqdm
from utils import split_file_name, shorten_path, clean_up_word_list

from database import (
    setup_database,
    create_session,
    File,
    Word,
    add_file_with_words,
    file_indexed,
)

dirs_to_ignore = [".git", "node_modules"]


def scantree(path: str):
    for entry in os.scandir(path):
        if entry.is_dir(follow_symlinks=False) and entry.name not in dirs_to_ignore:
            yield from scantree(entry.path)
        else:
            yield entry


if __name__ == "__main__":
    path_to_scan = "/Users/kavidey/Downloads"

    # print(splitFileName("testTest.a_b c"))

    engine = setup_database()
    session = create_session(engine)

    with tqdm(scantree(path_to_scan)) as entries:
        for entry in entries:
            try:
                if entry.is_file:
                    entry = Path(entry)
                    entries.set_description(shorten_path(str(entry)))
                    if not file_indexed(session, entry.stem, str(entry.parent)):
                        # if mimetypes.guess_type(entry)[0] == "text/plain":
                        f = File(
                            filename=entry.stem,
                            content="",
                            path=str(entry.parent),
                            filetype=entry.suffix,
                            modified_date=datetime.datetime.fromtimestamp(
                                entry.stat().st_ctime
                            ),
                        )
                        title_keywords = split_file_name(entry.stem)
                        title_keywords = clean_up_word_list(title_keywords)
                        add_file_with_words(session, f, title_keywords)
            except:
                pass

    session.commit()
