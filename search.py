# %%
import utils
from database import (
    setup_database,
    create_session,
    File,
    Word,
    add_file_with_words,
    file_indexed,
)
import timeit

# %%
def get_search_terms(search_string: str):
    search_terms = utils.nlp(search_string.lower())

    return [token.lemma_ for token in search_terms if not token.is_stop]


# %%
engine = setup_database()
session = create_session(engine)
# %%
search_string = "E80"

def search_files(search_string: str):
    search_terms = get_search_terms(search_string)

    matches = {}
    files = set()
    for term in search_terms:
        w = session.query(Word).filter_by(word=term).first()
        if w:
            for f in w.files:
                files.add(f.id)
                if f.id in matches:
                    matches[f.id] += 1
                else:
                    matches[f.id] = 1
    sorted_files = sorted(
        files, key=lambda f: session.query(File).filter_by(id=f).first().modified_date
    )
    sorted_files = sorted(sorted_files, key=lambda f: matches[f])
    sorted_files.reverse()

    return sorted_files
# %%
top_k = 10

for f in search_files(search_string):
    print(session.query(File).filter_by(id=f).first().filename)
# %%timeit search_files(search_string)
# %%
from ipywidgets import interact
from IPython.display import display, clear_output
# %%
@interact
def show_matches(search_string='resume'):
    clear_output(wait=True)

    i = 0
    for f in search_files(search_string):
        if i >= 10:
            break
        
        print(session.query(File).filter_by(id=f).first().filename)
        i += 1
# %%
