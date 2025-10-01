---
title: "An Afternoon Reading the Dictionary"
description: |
    I scripted the extraction of dictionary definitions from macOS' built-in Oxford Dictionary archive to automate my new-vocabulary blog page. As all good automation projects, I probably spent more time automating the process than it would've taken to do it manually.
tags: ["words"]
date: "2025-08-09"
---

I'm a heavy user of Apple's [Look Up for words](https://support.apple.com/en-au/guide/mac-help/mchl3983326c/mac) on macOS. With a quick [force click](https://support.apple.com/en-au/guide/mac-help/mchl3983326c/mac) you can instantly summon the dictionary or thesaurus for a selected word, which removes the friction of pursuing "hmm, that's a new word - what does it mean?"

I’ve been collecting new words I discover in an Apple Note for years and recently started migrating my list to a [living blog page](https://abstractnonsense.xyz/blog/2025-08-01-a-living-log-of-lexical-learnings/). Rather than manually collecting the macOS built-in _Oxford Dictionary_ definition for each word (of which I have hundreds), I thought I'd script it. It turns out that Oxford Dictionaries has a very neat [API](https://developer.oxforddictionaries.com/documentation), but unfortunately it's exorbitantly [expensive](https://account.oxforddictionaries.com/pricing).

Poking around some more, I noticed an [Apple Shortcut](https://support.apple.com/en-au/guide/shortcuts-mac/apdf22b0444c/mac) called `Show definition of`, but it just opens `Dictionary.app` with a deep link to the word entry.

But after some [searching](https://apple.stackexchange.com/questions/404461/where-are-dictionaries-stored-in-macos), I uncovered the path to the OED dictionary database on my Mac and a wonderfully detailed [blog post](https://fmentzer.github.io/posts/2020/dictionary/) and [GitHub repo](https://github.com/JadedTuna/apple-dictionary) explicating how to extract the entries from the encoded dictionary files.

After munging around with XPath queries on the resulting `dictionary.xml` file, I settled on the following `xmlstarlet` snippet that extracts out the definition and example for each headword in `new-word-list.txt` and formats it as a YAML list:

```shell

# Construct an XPath filter expression for each headword
xpath_expr="//d:entry["
first=1
while read word; do
  if [ $first -eq 1 ]; then
    xpath_expr="${xpath_expr}@d:title='${word}'"
    first=0
  else
    xpath_expr="${xpath_expr} or @d:title='${word}'"
  fi
done < new-word-list.txt
xpath_expr="${xpath_expr}]"

# and extract the definitions and examples for each headword
xmlstarlet sel -N d="http://www.apple.com/DTDs/DictionaryService-1.0.rng" \
  -t \
  -m "$xpath_expr//span[starts-with(@class, 'msDict') and .//span[@class='df']]" \
  -o "- word: \"" \
  -v "ancestor::d:entry/@d:title" \
  -o "\"" -n \
  -o "  definition: \"" \
  -v "normalize-space(.//span[@class='df'])" \
  -o "\"" -n \
  -o "  example: \"" \
  -v "normalize-space(.//span[@class='eg']//span[@class='ex'])" \
  -o "\"" -n \
  -n \
  dictionary.xml > vocabulary.yaml
```

This YAML definition list gets popped into a [`data/vocabulary.yaml`](https://github.com/stochastical/abstractnonsense/blob/main/data/vocabulary.yaml) file and a [`layouts/_shortcodes/vocabulary.html`](https://github.com/stochastical/abstractnonsense/blob/main/layouts/_shortcodes/vocabulary.html) Hugo shortcode constructs the entries in the blog post at build time. From here on, if I want to add a new word it's trivial to simply update the vocabulary list manually with the word and definition.

I probably spent more time working out how to automate this process ([relevant XKCD](https://xkcd.com/1319/)) than it would've taken to do it manually, but I had good fun learning how to read the dictionary this afternoon.

> **An aside on copyright and fair use**: I went down an interesting little rabbit hole understanding how copyright and dictionaries relate. I am most definitely not a lawyer, but my understanding is that, whilst you cannot copyright the _definition_ of a word, the broader _composition_ of headwords with word senses, parts of speech, IPA transcription, examples, and other components commonly found in a dictionary is certainly copyrightable.
> However, considering I am replicating a vanishingly small proportion of the dictionary, and I am attributing appropriately, I believe this usage satisfies fair use.
