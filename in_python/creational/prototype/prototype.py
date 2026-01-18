
from abc import ABC 
import copy 
from dataclasses import dataclass
from datetime import datetime 
from typing import Self 


class Prototype(ABC):
    def clone(self)-> Self:
        ... 


@dataclass
class Documento(Prototype):
    tittle: str
    author : str 
    content : str
    creation_date : datetime 
    
    def clone(self) -> Self : 
        clon = copy.deepcopy(self)
        clon.creation_date = datetime.now()
        clon.tittle = format(f"copia de {self.tittle}")
        return clon 

        
    

