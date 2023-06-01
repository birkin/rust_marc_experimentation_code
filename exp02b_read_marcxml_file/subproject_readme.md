# next

- get file-path from envar

- think about next steps...
    - store pieces to sqlite
        - do a lookup on the mmsid, to see if it's already in the db
        - if not, add it
        - if so, update it
        

---

# usage...

```
% cd ./exp02b_read_marcxml_file

% source ../../load_envars/exp02b_read_marcxml_file.sh
```

...and then either:

```
% cargo run
```

...or: 

```
% ./target/release/exp02b_read_marcxml_file
```

---


# misc notes...

## async experimentation

(I love thinking about and experimenting with concurrency.)

Just expored rayon a bit after reading this matsakis post, ["Rayon: data parallelism in Rust"](https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/) -- and am removing all the default async code.

Running the updated code on the 10,000 item file, _not_ using any async for the process_file() calls to it's component functions, but instead using rayon on the full iteration through all the records, I get a slight improvement:

- iterate through records via rayon: `./target/release/exp02b_read_marcxml_file  2.33s user 0.17s system 102% cpu 2.485 total`


~~As an exercise, I'm going to write this single-threaded, but make all function-calls async (with awaits), to experiment with forcing myself to keep concurrency in the back of my mind. Also, I'll be curious to see whether I have to change any practices via the compiler. Then, hopefully, I can go back and implement concurrency where it would be beneficial.~~

An initial version of this, compared to the version with no async references (run on the 10,000 item file), is a bit slower, but ok:

- async definitions only: `./target/release/exp02b_read_marcxml_file  2.44s user 0.07s system 99% cpu 2.5355 total
`

- no async references: `./target/release/exp02b_read_marcxml_file  2.43s user 0.08s system 99% cpu 2.5285 total`


## async experimentation -- other

As an experiment, I tried making process-records async, [here](https://github.com/birkin/rust_marc_experimentation_code/blob/asyc_02b_try/exp02b_read_marcxml_file/src/main.rs#L40-L43).

The overhead made it slower:

- taking four runs, dropping the high and low, and averaging middle-two...

- async:

        ./target/release/exp02b_read_marcxml_file  2.50s user 0.07s system 99% cpu 2.6005 total

- single-threaded:

        ./target/release/exp02b_read_marcxml_file  2.43s user 0.08s system 99% cpu 2.5285 total

---


# author possibilities

Attribution
edited by A. Yemisi Jimoh and Françoise N. Hamlin.

that comes from...

		<datafield tag="245" ind1="0" ind2="0">
			<subfield code="a">
				These truly are the brave :
			</subfield>
			<subfield code="b">
				an anthology of African American writings on war and citizenship /
			</subfield>
			<subfield code="c">
				edited by A. Yemisi Jimoh and Françoise N. Hamlin.
			</subfield>
		</datafield>

---

Creator
Jimoh, A. Yemisi, 1957- editor. 
Hamlin, Françoise N., editor. 

that comes from...

		<datafield tag="700" ind1="1" ind2=" ">
			<subfield code="a">
				Jimoh, A. Yemisi,
			</subfield>
			<subfield code="d">
				1957-
			</subfield>
			<subfield code="e">
				editor.
			</subfield>
		</datafield>
		<datafield tag="700" ind1="1" ind2=" ">
			<subfield code="a">
				Hamlin, Françoise N.,
			</subfield>
			<subfield code="e">
				editor.
			</subfield>
		</datafield>


---

for comparison, the 3 relevant fields for this ZMM entry from:
<https://bruknow.library.brown.edu/discovery/fulldisplay?docid=alma991033548039706966&context=L&vid=01BU_INST:BROWN&lang=en>

100	1#$aPirsig, Robert M. 

245	10$aZen and the art of motorcycle maintenance: $ban inquiry into values, $cby Robert M. Pirsig. 

600	10$aPirsig, Robert M. 

---

target title: "These truly are the brave : an anthology of African American writings on war and citizenship"

---

# bib-info

Ok, so the bibs are in:

		<datafield tag="907" ind1=" " ind2=" ">
			<subfield code="a">
				.b10737662
			</subfield>

...and they include the check-digit

The hierarchy is <collection> (for full-set), then <record> -- and then <datafield>.

The mmsid for that is: `991043515515806966`, found in...

		<controlfield tag="001">
			991043515515806966
		</controlfield>

---


this bib-record:
<https://bruknow.library.brown.edu/discovery/fulldisplay?docid=alma991043515515806966&context=L&vid=01BU_INST:BROWN&lang=en>

---
---

exploring xml for:
<https://bruknow.library.brown.edu/permalink/01BU_INST/9mvq88/alma991043515515806966>

for previous bib work, see:
`bib_redirect_stuff/bib_redirect_code/bib_redirector`

looks like a standard bib is b1234567, and b12345678 with a check-digit ('8' there isn't algorithmically correct).

---

# Explanations...

## Prompt

this Rust line works...

title = subfield.value.clone().unwrap_or_else(|| "".to_string());

...and this doesn't:

title = subfield.value.unwrap_or_else(|| "".to_string());

...with the help text explaining:

cannot move out of `subfield.value` which is behind a shared reference
move occurs because `subfield.value` has type `std::option::Option<std::string::String>`, which does not implement the `Copy` trait

Can you add to that explanation, including an explanation of the "shared reference" note?

## Response

Certainly. The error is indicating that Rust is trying to move `subfield.value` (an `Option<String>`) but can't because it's behind a shared reference. 

To explain this further, let's discuss the different types of references and the concept of ownership in Rust:

1. **Shared References (`&T`):** A shared reference in Rust allows read-only access to a value of type `T`, and you can have as many shared references to a value as you want. The key point is that you cannot mutate the value through a shared reference. 

2. **Mutable References (`&mut T`):** A mutable reference in Rust allows read-write access to a value of type `T`. The rule with mutable references is that you can only have one mutable reference to a particular piece of data in a particular scope.

3. **Ownership and `move`:** In Rust, each value has a variable that owns it. When the owner goes out of scope, Rust will automatically clean up the value. The concept of `move` is related to this ownership model. When you assign a value to another variable or pass it to a function, Rust will move the value instead of copying it (for types that don't implement the `Copy` trait). After the move, the original variable can no longer be used.

The `unwrap_or_else` function needs to take ownership (i.e., it needs to perform a `move`) of the `Option<String>` to be able to return the contained `String` if it exists. However, in your second line of code, `subfield.value` is behind a shared reference (because you're inside a reference to `subfield`), so you're not allowed to move out of it, hence the error.

The reason the first line of code works (`title = subfield.value.clone().unwrap_or_else(|| "".to_string());`) is because `clone()` creates a copy of `Option<String>`, and `unwrap_or_else()` is taking ownership of this copy, not the original value. This is allowed in Rust, and it doesn't violate the borrowing rules.

I hope this explanation helps clarify the issue!

---
