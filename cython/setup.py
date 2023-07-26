from setuptools import setup
from Cython.Build import cythonize

setup(
    name="fibs",
    ext_modules=cythonize("fibs.pyx"),
)