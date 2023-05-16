# next

- title always seems to be 245/a.

- research the different possibilities for the author from info below.



- author possibilities...

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

Resources:
- <https://github.com/blackbeam/rust-marc>
- <https://github.com/lannonbr/marc_cli>
- <https://github.com/JacobSandin/marc_21>

---
