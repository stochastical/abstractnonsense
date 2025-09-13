---
title: "Can a regex match valid card numbers?"
date: "2025-08-31"
tags: ['mathematics', 'DFA', 'algorithms']
---

Sometimes the mere existence of a question is dangerous. A [colleague recently asked](/micro-blog/2025-03-11-irregular-expressions/) me whether it's possible to validate credit card numbers in regex using the [Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm), and, well, I was thoroughly [nerd-sniped](https://xkcd.com/356/). I'll outline the problem briefly and we'll try to explicate a solution together.

For what follows, it'll be helpful if you've encountered concepts such as [Deterministic Finite Automata (DFA)](https://en.wikipedia.org/wiki/Deterministic_finite_automaton), [Regular languages](https://en.wikipedia.org/wiki/Regular_language) and [modular arithmetic](https://en.wikipedia.org/wiki/Modular_arithmetic) before. If you haven't, I think a quick skim at the Wikipedia pages should be sufficient to follow. I'll also pair the mathematical formalisms with Python code in the exposition to make it easier to follow.

## What's in a card number, anyways?

Credit card numbers aren't just an unstructured string of digits. There's an [ISO standard](https://en.wikipedia.org/wiki/ISO/IEC_7812) that specifies the format of a card number to be as follows:
- A prefix of 6 or 8 digits: the [Issuer Identification Number (IIN)](https://en.wikipedia.org/wiki/Payment_card_number#Issuer_identification_number_(IIN)). This is the digit sequence that is often used to dynamically populate the appropriate card provider icon in payment modals.
- All but the last digit represent the individual account ID. This, along with the CVV and expiry date, is the really sensitive part of payment card details that you definitely don't want to disseminate on the internet. For any examples in this post, we'll be using dummy card numbers kindly [provided by Stripe](https://docs.stripe.com/testing#cards).
- And the last digit, which is most interesting to us, is the **check digit**, which is calculated using the Luhn algorithm from the other digits of the card number and is used to detect mistypes. This check digit is how payment screens catch typos without having to fire off any API request to a payment gateway to verify whether your card number is valid.

If you haven’t come across a check digit before, you can think of it like a *hash function.* The idea is that changing a digit (say accidentally, through a typo) should change the checkdigit and thus flag that an error has occurred.

## What is the Luhn algorithm?

A [typical formulation](https://rosettacode.org/wiki/Luhn_test_of_credit_card_numbers) of the Luhn algorithm can be described as follows: walk over the digits from *right-to-left*, alternating between adding the digit and adding the “*Luhn double*” of the digit to a rolling sum. At the end, we check whether this sum is divisible by 10.

The Luhn algorithm in Python is:

```python {hl_lines=[2]}
def validate_luhn_checkdigit(number: int) -> bool:
    digits: list[int] = [int(d) for d in reversed(str(number))]
    luhn_sum = 0
    for i, d in enumerate(digits):
        if i % 2 == 1:
            luhn_sum += luhn_double(d)
        else:
            luhn_sum += d
    return luhn_sum % 10 == 0
```

However, since DFAs can't walk a string in *reverse* order![1](fn "Regular languages are closed under reversal, but this is not sufficient to reverse a string to build our DFA. For instance, no DFA can recognise the language of palindromic strings."), we'll work with an equivalent *left-to-right* formulation based on the parity of the length of the digits:

```python {hl_lines=[3, 6]}
def validate_luhn_checkdigit(number: int) -> bool:
    digits: list[int] = [int(d) for d in str(number)]
    parity = len(digits) % 2
    luhn_sum = 0
    for i, d in enumerate(digits):
        if i % 2 == parity:
            luhn_sum += luhn_double(d)
        else:
            luhn_sum += d
    return luhn_sum % 10 == 0
```

The “*Luhn double*” function \(\ell: \texttt{[0-9]} \to \texttt{[0-9]}\) is defined as follows:

$$
\ell(d) = \begin{cases} 2d & 2d < 10 \\ 2d - 9 & 2d \ge 10 \end{cases}
$$

where \(\texttt{[0-9]}\) is a shorthand for the set of integers \(\{0, 1, \dots, 9\}\).

In Python this is equivalent to:

```python
def luhn_double(d: int) -> int:
    return 2*d if 2*d < 10 else 2*d - 9
    
>>> [(d, luhn_double(d)) for d in range(10)] 
[(0, 0),
 (1, 2),
 (2, 4),
 (3, 6),
 (4, 8),
 (5, 1),
 (6, 3),
 (7, 5),
 (8, 7),
 (9, 9)]
```

Notice that the reason we guard against erroneous transpositions with the “Luhn doubling” function is that it’s just a *permutation* function on the digits \(\{0, \dots, 9\}\). Meaning two distinct digit inputs are guaranteed to map to distinct outputs, and thus contribute to the rolling sum differently, mutating the checkdigit.

Please note that whilst DFAs are defined on alphabets and *strings*, we're defining the implementation here to accept an *integer*. That's ok, we'll be a bit informal and treat an integer as a digit-string. You may notice this ignores left-padding with zeroes (for instance, `02` is not a defined decimal literal in Python) or degenerate cases like empty strings.

## Why do we care about card numbers?

The question here is: does there exist a **regex** that computes the Luhn algorithm on a sequence of digits to match *valid* card numbers?

If you search for *"a regex to validate credit card numbers"*, you'll come across [StackOverflow answers](https://stackoverflow.com/a/23231321) that aggregate a set of patterns that match the prefix and length requirements for major card providers. But this is boring! Nothing there guarantees that the *check digit is correct*!

> *A quick aside on **validity**.* From here on out, when I refer to validity or correctness, I'm purely referring to whether or not a number satisfies the Luhn checkdigit algorithm. We're not going to consider whether the card number corresponds to a real, active, and fit-for-payment account, we'll take that as an orthogonal consideration.

Now, before we get into the guts of this, let's pause for a second to consider why such an endeavour is useful to contemplate at all. First and foremost, it's a purely *interesting* question to ponder. I posit this is a necessary and sufficient condition to spend time on any question.

But if that's not enough for you: the [powers that be](https://en.wikipedia.org/wiki/Payment_Card_Industry_Security_Standards_Council) mandate a standard ([PCI DSS)](https://en.wikipedia.org/wiki/Payment_Card_Industry_Data_Security_Standard) by which you solemnly swear to handle card details. Many financial organisations will have special systems that are approved to store and process card details, and many, many more systems that are not. And never, ever shall the twain meet. But how do you make sure your logs aren't accidentally and surreptitiously exfiltrating sensitive payment details? Well, you definitely, definitely **should not** rely on a regex to detect or scrub such flagrant violations. But if you *wanted* to, *could* you?

## Working towards a solution

We can make the trivial observation that, as per the spec, payment card numbers have a maximum length. Since all *finite languages* are *regular*, that means it's possible to simply enumerate all possible valid card numbers and union them together with `|` into a mammoth regex. For those tempted to try, it’s worth appreciating the scale of such an endeavour. If we assume a card number has 16 digits, then there's \(10^{15}\) sequences that have a valid checksum at the end![2](fn "We're going to ignore leading zeroes or the prefix being one of a small set of possible IINs.").

But the more general question is the one that kept me up at night:

> Does there exist a DFA that recognises the language \(L\) of numbers written in base \(10\) that satisfy the Luhn checkdigit algorithm?

Note that this language is infinite, since we do not restrict the length of the numbers. If such a DFA exists, then the language is regular (by Kleene's theorem![3](fn "I propose we rename this to `Kl(e|n)*'s` theorem.")), and hence there *must* be a regular expression that matches all words![4](fn "Note for the programmer: in keeping with the parlance of calling these sets *languages*, elements of a language are often called *words* - this is interchangeable with *strings*.") in the language.

Now it's been a few years since I've touched DFAs or language theory, and at first, I didn't think this problem was solvable. I didn't think too hard about it until I stumbled across the excellent [Arithmancia Automatorum](https://iagoleal.com/posts/automata-divisibility/) which presents *"A Painfully Explicit Construction of The Minimal DFA for Divisibility"*. Understanding how to construct the transition function for a DFA that recognises the language of numbers written in base \(b\) that are divisible by \(m\) gave me hope that there existed a similar transition function to represent the Luhn algorithm as a DFA.

Interestingly, Luhn invented a *mechanical* device![5](fn "The original source of Luhn's algorithm is in a patent (*US patent 2950048A*), which has fortunately been scanned and made accessible as a [PDF via Google Patents](https://patentimages.storage.googleapis.com/ec/2a/f7/b9af046ed26128/US2950048.pdf).") to compute the check digit. If we can do it mechanically, surely this is begging to be implemented as a Discrete Finite Automaton!


## The construction of a DFA to recognise the Luhn algorithm

First we note that *a priori*, the DFA can not know the parity of the string. But that's ok! It just means we need to encode handling both *even* and *odd* length strings into the states of our DFA. The transition function must somehow 'update' the parity as the DFA consumes each digit.![6](fn "It might actually be easier to motivate the form of the transition function of the DFA at the cost of a more roundabout construction. Since we don’t know whether the string is even or odd at the start, we could traverse along an **NFA** that traces both even and odd paths together using epsilon transitions. And any NFA can be converted into a DFA!")

This motivates the following state construction. We can define two DFAs that recognise the Luhn algorithm: one that recognises the language of even-length strings, and another for the language of odd-length strings. Since regular languages are closed under union, we can then combine these two DFAs into a single DFA. The union construction involves taking the product of each state set and creating a tuple representing the union-state.

The second observation is that modular arithmetic distributes nicely with addition. This means we can compute a running partial Luhn-sum modulo 10 along our transitions without worrying about the final answer being invalid.

Let's formalise the above now. The following will get a bit mathematically involved - feel free to skip ahead to the final construction and accompanying code if you'd prefer.

Let the alphabet \(\Sigma = \texttt{[0-9]}\) be the set of base-10 digits and the state set \(\mathcal{S} = (E, O) = \texttt{[0-9]} \times \texttt{[0-9]}\) represent the partial Luhn-sum modulo 10 for even and odd-length strings, respectively.

We start our DFA from \(s_0 = (0, 0)\) and accept any state \(\{(E, O) \in \mathcal{S} \mid E = 0 \}\). To track our alternating partial Luhn sum as we alternate between string-length parities, we define the transition function \(\delta: \mathcal{S}\times\Sigma \to\mathcal{S}\) from a current state and new digit \(d\) to the next state as follows:

$$
\boxed{
    \delta((E, O), d) = ((O+d) \bmod 10, \, (E + \ell(d)) \bmod 10)
}
$$

where \(\ell\) is the *Luhn double* function as defined above. Note that the \(E\) and \(O\) partial sums swap between the pair in each transition!

To intuit why this works, notice that the transitions are computed *ahead-of-time*. We are defining the computation, and baking it into the DFA as a sort of computational graph. I think this is a wonderfully powerful notion - it demonstrates that a blind automaton![7](fn "or a very busy beaver") can perform non-trivial (but not arbitrary! ![8](fn "Dam(n), Busy Beavers can't do *everything*")) computations by simply following a rulebook of "if in state `X` and next symbol is `Y` then go to state `Z`". After all, isn't that exactly what a computer is? There's something profound about seeing the correspondence between mathematics and computer science at such a deep level.

In Python, we can define the DFA succinctly as:

```python
def luhn_dfa(number: int) -> bool:
    word: list[int] = [int(d) for d in str(number)]

    def transition(state: tuple[int, int], d: int) -> tuple[int, int]:
        (E, O) = state
        return ((O + d) % 10, (E + luhn_double(d)) % 10)
    
    state = (0, 0)
    for d in word:
        state = transition(state, d)
    return state[0] == 0
```

So there we have it! A DFA with 100 states and 1000 transitions that recognises the language of valid Luhn checkdigit words!

## How do we build the regex for this DFA?

I've somewhat buried the lede here. You probably clicked here expecting to see some giant regex you could copy-paste into a PII scanner and be off. After all, converting between a DFA that recognises a language and a regular expression that matches all words in the same language is mathematically assured. And we've constructed the DFA! Unfortunately, conversion procedures between a DFA and the corresponding regex often lead to an *exponential* combinatorial explosion.

Now, an earlier version of this post didn't think that this conversion would be computationally tractable. But, I wasn't done tilting at windmills! After a email and code exchange with the wonderfully helpful [Alok Menghrajani](https://www.quaxio.com), who pointed me to the regular expression manipulation library [greenery](https://github.com/qntm/greenery), I have a regex!

Or, more correctly, the following code defines a DFA for both even and odd-length Luhn-valid strings ![*](fn "Of course, once you have both regexes, you can union them together with `|` for a single regex."), and uses `greenery`'s [implementation](https://github.com/qntm/greenery/blob/e55c96712677d56ef14664a1595a47fb7f26bc01/greenery/rxelems.py#L260C4-L260C4) of the Brzozowski algebraic method to convert the DFA to a regex.

For the intrepid explorer who's made it this far, be warned that the resulting regex pairs are *enormous*. It takes ~20 minutes per even/odd expression computation on my M1 Air, and the resulting regexes are `32,461,605`, `48,236,673` characters long, respectively.

```python
# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "greenery",
# ]
# ///

import time
from pathlib import Path

import greenery


def build_luhn_dfa(parity: int) -> greenery.fsm.Fsm:
    def luhn_double(d: int) -> int:
        return 2 * d if 2 * d < 10 else 2 * d - 9

    digits = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'}
    non_digits = ~greenery.Charclass((('0', '9'),))
    alphabet = {greenery.Charclass(c) for c in digits}
    states = range(0, 2 * 10)
    initial = 0 if parity else 10
    finals = {10}

    transition_map = {}
    for from_state in states:
        state_transition = {}
        for c in digits:
            d = int(c)
            next_state = -1
            if from_state < 10:
                next_state = (d + from_state) % 10 + 10
            else:
                next_state = (luhn_double(d) + from_state - 10) % 10
            state_transition[greenery.Charclass(c)] = next_state
        state_transition[non_digits] = 0
        transition_map[from_state] = state_transition

    return greenery.fsm.Fsm(
        alphabet=alphabet | {non_digits},
        states=states,
        initial=initial,
        finals=finals,
        map=transition_map,
    )


if __name__ == "__main__":
    # Even-length strings
    print('== Even-length Luhn strings ==')
    luhn_dfa_even = build_luhn_dfa(0)
    print(luhn_dfa_even)

    # Build and save regex
    start = time.time()
    regex_even: greenery.Pattern = greenery.rxelems.from_fsm(luhn_dfa_even)
    print(f'Built regex for decimal even-length strings in {time.time() - start:.1f}s')
    regex_str_even = str(regex_even)
    print(f'{len(regex_str_even)=} with {regex_str_even[:100]=}')
    Path('luhn-regex-even.txt').write_text(regex_str_even)

    # Odd-length strings
    print('== Odd-length Luhn strings ==')
    luhn_dfa_odd = build_luhn_dfa(1)
    print(luhn_dfa_odd)

    # Build and save regex
    start = time.time()
    regex_odd: greenery.Pattern = greenery.rxelems.from_fsm(luhn_dfa_odd)
    print(f'Built regex for decimal odd-length strings in {time.time() - start:.1f}s')
    regex_str_odd = str(regex_odd)
    print(f'{len(regex_str_odd)=} with {regex_str_odd[:100]=}')
    Path('luhn-regex-odd.txt').write_text(regex_str_odd)
```

In case you're wondering, is this actually executable by any regex engine? I'm delighted to report that it sure is! Attempting to run it with `grep` or [`ripgrep`](https://github.com/BurntSushi/ripgrep) results in the process being killed from memory exhaustion. But Python's [`re` module](https://docs.python.org/3/library/re.html) can handle it just fine!

In some sense, DFAs can be more compact for problems like these as they can store arithmetic state within their states and transitions. Regular expressions don't permit arbitrary transitions in the same way - though the two are equally powerful in expressiveness, in the formal sense.

## What's next?

1. I'll update this post with a DOT graph visualisation of this DFA (once I can figure out how to get Graphviz to layout the transitions nicely)
2. I'll add in a sketch of a proof by induction that the DFA is indeed *correct*
3. We'll sketch out or prove why this DFA is *minimal*
4. Add some Haskell code too for no reason other than Haskell is clean and beautiful
5. Probably fix some typos since I'm sure I've made a bunch of mistakes..
6. Add in some walked examples to illustrate the execution of the DFA (and hopefully link to [Automatarium](https://automatarium.tdib.xyz) for a live example!)
7. Are there ways to quantify bounds on the size of regexes and DFAs for languages? Is there a theory of [Lexical density](https://en.wikipedia.org/wiki/Lexical_density) for formal languages in the Chomsky hierarchy?

Any errors are entirely my own - if you notice any mistakes, think of any interesting insights, or just want to chat, please reach out!
