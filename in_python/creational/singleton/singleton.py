
from dataclasses import dataclass



@dataclass
class Configuration: 
    _instance = None 
    _initialized = False

    program_name : str
    version : str

    def __new__(cls, *args, **kwargs):
        if not cls._instance:
            cls._instance = super(Configuration, cls).__new__(cls)
        return cls._instance
    
    def __init__(self):
        if self._initialized:
            return 
        # in production, load configuration from file or environment 
        self.program_name = "MyApp"
        self.version = "1.0.0"
        self._initialized = True
        