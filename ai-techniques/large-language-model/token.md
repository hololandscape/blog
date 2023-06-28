# Token

## Overview

In the context of LLM, ***tokens are the basic units of text or code that an LLM AI uses to process and generate language***. 

***Tokens can be characters, words, subwords, or other segments of text or code, depending on the chosen tokenization method or scheme***. 

## Tokenization

***Tokenization is the process of splitting the input and output texts into smaller units that can be processed by the LLM AI models.*** Tokens are assigned numerical values or identifiers and are arranged in sequences or vectors, and are fed into or outputted from the model. **The number of tokens parameter allows you to set a limit to how many tokens are generated.** The natural limit to the number of tokens the model can produce varies depending on the model size, with smaller models going up to 1024 and larger models going up to 2048. It is generally recommended to generate in short bursts instead of one long burst to avoid the model going off in a direction you are not expecting.

## What does tokenization have to do with the cost of running a model?
Tokenization affects the amount of data and the number of calculations that the model needs to process. ***The more tokens that the model has to deal with, the more memory and computational resources that the model consumes.*** Therefore, the cost of running an OpenAI or Azure OpenAI model depends on the tokenization method and the vocabulary size that the model uses, as well as the length and complexity of the input and output texts. Based on the number of tokens used for interacting with a model and the different rates for different models, your costs can widely differ. For example, as of February 2023, the rate for using Davinci is $0.06 per 1,000 tokens, while the rate for using Ada is $0.0008 per 1,000 tokens. The rate also varies depending on the type of usage, such as playground, search, or engine. Therefore, tokenization is an important factor that influences the cost and the performance of running an OpenAI or Azure OpenAI model.
