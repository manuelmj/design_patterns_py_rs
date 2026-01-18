import logging 
from datetime import datetime 
import pytest 
from creational.prototype.prototype import Documento 

logging.basicConfig(level=logging.info)
logger = logging.getLogger(__name__)



def test_prototype_document(): 
    
    original_document = Documento(
        tittle= "proof document",
        author= "manuel Manjarres",
        content = "proof information in this document",
        creation_date= datetime.now()
    )

    clone_document = original_document.clone()
    logger.info(f"Original Document: {original_document}")
    logger.info(f"Cloned Document: {clone_document}")
    
    assert(isinstance(clone_document, Documento))
    assert(clone_document.tittle == f"copia de {original_document.tittle}")
    assert(clone_document.author == original_document.author)
    assert(clone_document.content == original_document.content)
    assert(clone_document.creation_date != original_document.creation_date)
    assert(clone_document.creation_date > original_document.creation_date)







