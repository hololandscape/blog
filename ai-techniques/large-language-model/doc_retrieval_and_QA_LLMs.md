---
description: Generative AI - Document Retrieval and Question Answering with LLMs(Apply LLMs to the domain-specific data)
---


## Overview

With LLMs, we can integrate domain-specific data to answer questions. This is especially useful for data unavaliable to the model during its initial training, like a company's internal documentation or knowledge base.

Implementaion code [Todo](#Todo)

The **architecture** is called Retrieval Augmentation Generation or (Generative Question Answering). Below we will go through implement this architecture using LLMs and a Vector Database.

Here are some use cases:
* It reduce the time we need to interact woth documents
* There is no need for us anymore to search for answers in search results
* The LLM takes care of precisely finding the most relevant documents and using them to generate the answer right from the documents

## Preparing
* Lanchanin(or other chain)
* Documents
* Vector Database

### LangChain

It is used for tasks like Data Loading, Document Chunking, Vector Stores, and Text and Embedding model interaction. Here is more detail about [Text embedding models](https://python.langchain.com/docs/modules/data_connection/text_embedding/)


### Documents

Here the example we use the full Google Cloud Vertex AI Documentation https://cloud.google.com/vertex-ai/sitemap.xml Any type of document is possible.


### Combining LLMs and Vector Databases for Enhanced Question Answering Systems

Let's examine how we can combine the strengths of LLMs and Vector Databases to create a powerful document retrieval and question answering system.

<figure><img src="https://miro.medium.com/v2/resize:fit:2000/format:webp/1*EeU9ajmZp5LNeJg0xCHueQ.png" width="1000"><figcaption><p>Source:https://medium.com/google-cloud/generative-ai-document-retrieval-and-question-answering-with-llms-2b0fb80ae76d</p></figcaption></figure>


## Step involved
* Getting the documents and preprocess

### Loading data

Using LangChains sitemap document loader to load the documents and split them into smaller chunks. **It reduces the size of the text that is sent to the LLM** and it has two main advantages:

* The embedding that is created for that document chunk more accurately represented the information to our question.
* During retrieval, we can receive smaller chunks and keep the LLM contenxt small. This leads to faster latency and lower costs.

After this process, we have a folder containing a bunch of chunk documents. Those chunk documents are used in the next steps to:
* Create the document embeddings
* Answer our question

### Document Embedding

**The embedding step transforms our documents into a vector representation called embedding**. We can use an LLM to encode each document chunk into a high-dimensional vector. This process captures the semantic information of the document in the form of a vector. Here is an example:

```python
from langchain.embeddings import xxxx

embeddings = xxxx()

text = 'xxxx'
doc_result =enbeddings.encode([text])
```

After this process, we have a for each document chunk a vector representation.

### Storing in Vector Database

It is helpful a vector database enables efficient similarity search among the vectors, helping us retireve the most relevant documents for a given query.

### Query Processing

When the query comes in, the LLM to transform the question into a vector.

### Document Retrival

Based on the query from the previous step, we search the vector database for the most semantically similar documents vectors to the question vector. The documents associated with these vectors are the most relevant to our query and will be as context for our LLM.

The vector database only contains the embeddings and an identifier without the actual text. Using LangChain to identify and matches the vector results to the actual documents.

### Answer Generation

The retrieved documents are fed back as context into the LLM. The LLM generates an answer based on the information provided. The important part is the prompt structure.


## Tuning vs. Indexing

Either fine-tune the model or LLM use an external index that can be queried at runtime. Indexing has some advanatges over tuning:
* New documents can be added to the index without retraining the model(In a real time)
* We [circumvent the context size limitations](#document-embedding)
* Restricted documents at anytime
* Cheaper
* Explainable
* Combined with prompt engineering


## Conclusion

This article is a showcase for recolutionize document retrieval and question-answering systems by harnessing the power of LLMs when combined with Vectr Databses.


## Credit
https://medium.com/google-cloud/generative-ai-document-retrieval-and-question-answering-with-llms-2b0fb80ae76d