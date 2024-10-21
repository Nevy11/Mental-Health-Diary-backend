from haystack.nodes import FARMReader, BM25Retriever
from haystack.document_stores import InMemoryDocumentStore
from haystack.pipelines import ExtractiveQAPipeline
from transformers import AutoModelForQuestionAnswering, AutoTokenizer
from ffi_content import my_contents

# Define global variables for the retriever and pipeline
document_store = None
retriever = None
pipeline = None


def initialize_pipeline():
    global document_store, retriever, pipeline
    # Initialize document store and retriever
    document_store = InMemoryDocumentStore(use_bm25=True)
    document_store.write_documents(my_contents)
    retriever = BM25Retriever(document_store=document_store)

    # Load the reader model from the local cache
    reader = FARMReader(model_name_or_path="deepset/roberta-base-squad2")

    # Build the extractive QA pipeline
    pipeline = ExtractiveQAPipeline(reader=reader, retriever=retriever)
    print("Pipeline initialized.")


def ask_question(question):
    if pipeline is None:
        return "Pipeline not initialized."
    answers = pipeline.run(
        query=question, params={"Retriever": {"top_k": 5}, "Reader": {"top_k": 1}}
    )

    if answers and "answers" in answers and len(answers["answers"]) > 0:
        return answers["answers"][0].answer
    return "No answer found."
