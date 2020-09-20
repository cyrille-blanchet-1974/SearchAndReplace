# SearchAndReplace
Search a string in a file and replace id with another string<BR>
<BR>
<BR>
syntax : search_and_replace /search:search_string /replace:replace_string /fic:file [/only_first] [/keep_old]<BR>
/search parameter gives the string to search (may be between ")<BR>    
/replace parameter gives the string to put instead<BR>
/fic parameters gives the file to work with<BR>    
if /only_first is set, only first searched string is replace event if present many times <BR>
if /keep_old the file before changes is kept (with a .old extention)<BR>
<BR>
Technical note: works with 3 threads <BR>
    -one to read<BR>
    -one to search/replace 
    -one to write<BR>
search is case sensitive    
