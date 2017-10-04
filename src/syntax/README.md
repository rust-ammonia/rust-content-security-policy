Parse the Content-Security-Policy directive syntax
==================================================

Built with <https://github.com/nikomatsakis/lalrpop>.

What this parser does:

* converts from the textual syntax to a struct of slices into the original buffer
* handles the first-declaration-wins rule of the CSP spec
* strips unknown and invalid declaration types
* distinguishes between different types of source-expression

What this parser doesn't do:

* URI parsing/validation
* mimetype parsing/validation
