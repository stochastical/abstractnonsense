---
title: "How do Structured Outputs work?"
date: 2025-08-19
tags: ['LLM', 'mathematics']
draft: true
description: |
    Understanding the mathematics behind structured outputs in language models. How token generation is constrained to produce valid JSON and conform to schemas.
---

## Introduction

When you generate a response to a prompt with a language model, it doesn't return a *fixed* output. Instead, it generates a *distribution* over possible outputs, from which a specific output is sampled. Depending on some random state and the sampling strategy utilised, the output can 

Sometimes this is a feature. For instance, a standard way to exert some level of control over the output is to adjust the `temperature` parameter. This gives you a neat way to quantitatively adjust the level of randomness or 'creativity' in the models response. However, sometimes this is an anti-feature.

With the rise of Agentic coding (which as many have pointed out, is an ill-defined term, but I'll define here as simply 'LLM-backed code execution') and tool-calling, as a developer, you want the LLM to be capable of producing outputs that are both *relevant* and *structured* so that you can interpret its responses programmatically.

Hence, the advent of *Structured Outputs* (or 'constrained decoding'). OpenAI supports (though, interestingly, they don't promise that the response is **guaranteed** to always comply with the schema.)

When large language models were first introduced, a prompt-engineering 'trick' to produce structured outputs was to simply *ask* for them in the prompt. You'd specify a schema and a 'few-shot' set of examples in the hope you'd sufficiently biased the network ~~nudged~~ towards generating the desired output format. If your response was syntactically invalid or didn't adhere to the schema, you'd simply discard it and try again (much like rejection sampling).

```text
Pretty please will you give me a JSON response to this query. I would greatly appreciate it if you would indulge me and grant me the following response schema. I have attached the following set of examples below - feel free to peruse at your leisure. Thank you in advance.
```

*^ ancient incantation to please the LLM gods*

This ... works, but it's grossly inefficient. Every chat-completion API call is burning through cash, time, and more importantly, developer sanity. As Raymond Hettinger always insists, "there must be a better way".

## An introduction to logits

To understand how this works, we first need to understand a little about how LLMs generate a response to their prompt. I won't re-hash the whole explanation here (you can see XX for fantastic introductory pieces), but essentially LLMs generate a distribution over possible token outputs, for the entire vocabulary, from which a specific output is sampled. There's an excellent compilation of various sampling strategies **here**.

The output of the last layer of an transformer network will look something like:

```text
logits = [logit_1, logit_2, ..., logit_n]
```

with the softmax function producing a probability distribution over each token in the vocabulary. (If you'd like a great visualisation of how this works, I recommend checking out **this** resource.)

Now, the simplest possible sampling strategy is to simply pick the token with the highest probability. That is, if we describe the $\textsf{LLM}$ as as a discrete probability mass function conditioned on the previous tokens $\textsf{token}_1, \dots, \textsf{token}_n$ that maps from a token $t$ in the vocabulary $V$ to a probability in $[0,1]$, then we can write:

$$
\textsf{token}_{n+1} = \arg\max_{t \in V} \textsf{LLM}(t \mid \textsf{token}_1, \dots, \textsf{token}_n)
$$

as our sampling strategy.

For convenience, we often work with **logits** produced by the LLM. You can think of these as the 'raw' outputs of the LLM, before they get fed into the softmax and get turned into nice, clean probabilities. Logits have nice properties - since they are unbounded and not required to sum to 1, we can directly massage their values without having to go and adjust the overall distribution. (At least, until the end - we can always apply softmax later to get a probability distribution.)

When people talk about 'vector embeddings' or 'latent spaces', ~~they are often referring to the high-dimensional representations of tokens (or sequences of tokens) that the model learns during training. These embeddings capture semantic information about the tokens and their relationships to one another.~~

Many libraries expose functionality to `output_logits` (except, frustratingly Ollama - see here for a long-standing issue).

> Incidentally, other than the model's *training data* and learned *weights*, the *logits* for a specific prompt are a closely guarded secrete. This makes up the fabled triadic moat of which we often speak. OpenAI restricts the number of output logits you can access from their chat-completion API. It has been observed that using logits, you can (distill?) models. (See the Deepseek saga). In fact, there's a rich cornucopia of experiments you can run using a models, a source of endless interest to me (for instance - one token too poison them all...). 

But there's no reason to limit ourselves to simple $\arg\max$ing! Why don't we directly manipulate the logits themselves? For example, authors of yore used to write lipograms - texts written omitting a specific letter (the most famous of these is ??). Now, because of byte-pair encoding, a single letter may not be represented by a single token, but say we wanted our model to be particularly truculent, (or Australian), we could define a set of tokens that we would like to bias for or against. Prompting a model with this token set passed in, we return this delightful prompt-response pair:

```text
{{placeholder}}
User: What is the capital of France?
Assistant: The capital of France is Paris.
```

~~Going one step further, we can also manipulate the logits to achieve more complex behaviors. For instance, we could use techniques like **logit masking** to encourage the model to avoid certain tokens or to prefer others. This allows for a high degree of control over the model's output.~~

Taking this idea to its extreme, what if we biased the model at every step to *avoid* syntactically invalid tokens, for a given JSON schema?

~~Note that this is not just about avoiding invalid tokens, but also about encouraging valid ones. By shaping the logits in this way, we can guide the model towards producing outputs that are not only syntactically valid but also semantically meaningful.~~

Note that guaranteeing valid JSON outputs is an immediately useful thing to want to do, but it's by no means the only specification you can enforce. In fact, as we'll see shortly, we can reduce a JSON schema to a Regular Expression and enforce a much wider set of 

## A working example in Python

There are many libraries that implement this, but we’re going to use Outlines for this explanation. I like the way the code is structured, it’s nearly compartmentalised, and it wraps over the usual contingent of LLM frameworks (`Transformers`, `Ollama`, `llama-cpp`).

Now naturally you might wonder why this means we get good results at all. The output might be syntactically valid, and hold up to the schema, but why does this mean it’s semantically good? Well, LLMs now are pretrained on massive JSON examples. I suppose this means that JSON is truly the Lingua Franca of the human-interpretable/readable and machine parsable world. So the LLMs are already biased to good JSON, the forced decoding here merely ensures they comply 100% of the time.

Step 1: We take a JSON schema and convert it into a DSL that Outlines has defined

Step 2: We 'compile' the DSL into a regex

Step 3: We drop down to `rust` to compute an `Index`, ~which is effectively a set of permissible tokens that comply with the regex~

Step 4: We use this `Index` to guide the model's output by biasing the logits, ensuring it adheres to the specified JSON schema. (is this how it actually does it?) Note that Outlines sets the logits for impermissible tokens to -Inf to ensure they will never get sampled.

I've used GitHub permalinks here so they should exist for posterity :)

## Question - what if we stress test this?

Experiment idea: can we fuzz the prompt or schema and see if we can break `Outlines`?
Is it accurate when it says 100% success rate?

## A note on expressivity

There are certain classes of constraints that it would be impossible to enforce using just a JSON schema. ~~For example, consider a constraint that requires a specific relationship between two fields in the JSON object. While a JSON schema can enforce individual field types and structures, it cannot easily express complex inter-field relationships.~~ If we were to encode (a mathematical impossibility? or enforce a certain output length exceeding a models maximum output token count)

## On visualisations and interactive components

There are some prototype visualisation tools I've got sitting around that I'd love to put up in some form. Unfortunately, I don't have the time to polish them into a presentable state. This blog is statically generated from markdown using Hugo. I'd love to embed some React components here for expository purposes, but Hugo doesn't support MDX, and I don't . I'll think about how I can embed these here at some point and either update this post, or make a new post :).


## Conclusion
Thanks for reading. If you enjoyed this post, feel free to jump in on the Hacker News discussion, create a GitHub discussion, email me, or follow along the RSS feed.

---

Let’s highlight these log probs using a local model … And recall, t

First up, let’s borrow the simple example from the docs 

To understand the next part, we need to understand better some of the maths behind LLM code generation. If you aren’t familiar with this, that’s ok, you can use these great posts as background reading and come back here later 

We’re going to use Python throughout (except where we drop down into some Rust) but the maths principles apply throughout 

We’re going to use this example Pydantic schema throughout  and take a whirlwind tour through DFAs

In some sense, this is where the magic happens. The Index takes a long time to construct, which is why 

What about diffusion models you ask? That’s an excellent question and …

All the example code here is publicly available at. If you have `uv` installed, you can run a minimal example `uv run https:...`. Warning: please note that this will install a bunch of dependencies and download a (small, ~xxMB) model locally!