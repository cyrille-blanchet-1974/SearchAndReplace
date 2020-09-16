@echo off
cargo build --release
set prg=.\target\release\search_and_replace.exe
echo 1234567890abcdefghijklmnopqstuvwxyz > ori
echo 1234567890abcdefghijklmnopqstuvwxyz >> ori
echo 1234567890abcdefghijklmnopqstuvwxyz >> ori
echo 12345678901234567890 >> ori
copy ori tst1.txt
echo original:
echo -------------------------------------------
type ori
echo -------------------------------------------
echo Expected: change only 1st line
%prg%  /search:"1234567890" /replace:"0123456789" /fic:tst1.txt /only_first
echo -------------------------------------------
type tst1.txt.chg
echo -------------------------------------------
pause
echo 
echo 
copy ori tst2.txt
echo Expected: changeall lines (twice on the 4th)
%prg%  /search:"1234567890" /replace:"0123456789" /fic:tst2.txt
echo -------------------------------------------
type tst2.txt.chg
echo -------------------------------------------
pause
echo 
echo 
copy ori tst3.txt
echo Expected: error no file
%prg%  /search:"1234567890" /replace:"0123456789" 
pause
echo 
echo 
echo Expected: error no search
%prg%  /replace:"0123456789" /fic:txt3.txt
pause
echo 
echo 
echo Expected: error no replace
%prg%  /search:"1234567890" /fic:txt3.txt
pause
echo 
echo 
echo Expected: error file do not exist
%prg%  /search:"1234567890" /replace:"0123456789" /fic:tst99.txt
pause
echo 
echo 
echo Expected: error search equals replace
%prg%  /search:"1234567890" /replace:"1234567890" /fic:tst99.txt
pause
echo 
echo 
echo Expected: no number left
%prg%  /search:"1234567890" /replace:"" /fic:tst3.txt
echo -------------------------------------------
type tst3.txt.chg
echo -------------------------------------------
pause
echo 
echo 
copy ori tst4.txt
echo Expected: error no file
%prg%  /search:"1234567890" /replace:"12345678901234567890" /fic:tst4.txt
echo -------------------------------------------
type tst4.txt.chg
echo -------------------------------------------
pause