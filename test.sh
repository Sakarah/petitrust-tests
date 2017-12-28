#!/bin/bash

shopt -s nullglob

# script de test pour le projet de compilation

option=$1
compilo=$2
score=0
max=0
verbose=0


# echo "Test de $2"

echo

# tous les tests passent avec rustc
test_rustc() {
echo -n "syntax... "
for f in syntax/bad/*.rs; do
    if rustc $f -o a.out > /dev/null 2>&1 ; then
      echo "succès de rustc sur $f"; exit 1
    fi
done
echo "OK"

echo -n "typing... "
for f in syntax/good/*.rs typing/good/*.rs typing2/good/*.rs exec/*.rs exec-fail/*.rs; do
    rustc --emit=dep-info $f -o a.out  > /dev/null 2>&1 ||
     (echo "echec de rustc sur $f"; exit 1)
done
for f in typing/bad/*.rs typing2/bad/*.rs; do
    if rustc $f -o a.out > /dev/null 2>&1 ; then
      echo "succès de rustc sur $f"; exit 1
    fi
done
echo "OK"

echo "exec"
for f in exec/*.rs; do
    echo "  $f"
    expected=exec/`basename $f .rs`.out
    if rustc $f -o a.out > /dev/null 2>&1 ; then
      ./a.out > out
      if ! cmp --quiet out $expected; then
          echo "mauvaise sortie de rustc sur $f"; exit 1
      fi
    else
      echo "échec de rustc sur $f"; exit 1
    fi
done

echo "exec-fail"
for f in exec-fail/*.rs; do
    echo "  $f"
    expected=exec/`basename $f .rs`.out
    if rustc $f -o a.out > /dev/null 2>&1 ; then
      if ./a.out > /dev/null 2>&1 ; then
          echo "n'échoue pas sur $f"; exit 1
      fi
    else
      echo "échec de rustc sur $f"; exit 1
    fi
done

rm a.out
rm out
}

compile () {
if [[ $verbose != 0 ]]; then
  echo Compile $1 $2
  $compilo $1 $2;
else
  $compilo $1 $2 > /dev/null 2>&1;
fi;
}


# partie 1 : tests d'analyse syntaxique

partie1 () {

score=0
max=0

echo "Partie 1"

# les mauvais
echo -n "mauvais "
for f in syntax/bad/*.rs; do
    echo -n ".";
    max=`expr $max + 1`;
    compile --parse-only $f;
    case $? in
	"0")
	echo
	echo "ECHEC sur "$f" (devrait échouer)";;
	"1") score=`expr $score + 1`;;
	*)
	echo
	echo "ECHEC sur "$f" (pour une mauvaise raison)";;
    esac
done
echo

# les bons
echo -n "bons "
for f in syntax/good/*.rs typing/bad/*.rs typing/good/*.rs typing2/bad/*.rs typing2/good/*.rs exec/*.rs exec-fail/*.rs; do
    echo -n ".";
    max=`expr $max + 1`;
    compile --parse-only $f;
    case $? in
	"1")
	echo
	echo "ECHEC sur "$f" (devrait reussir)";;
	"0") score=`expr $score + 1`;;
	*)
	echo
	echo "ECHEC sur "$f" (pour une mauvaise raison)";;
    esac
done
echo

percent=`expr 100 \* $score / $max`;

echo -n "Syntaxe : $score/$max : $percent%"; }

# partie 2 : tests d'analyse sémantique


partie2 () {
echo
echo "Partie 2"

score=0
max=0

# les mauvais
echo -n "mauvais "
for f in typing/bad/*.rs; do
    echo -n ".";
    max=`expr $max + 1`;
    compile --type-only $f;
    case $? in
	"0")
	echo
	echo "ECHEC sur "$f" (devrait échouer)";;
	"1") score=`expr $score + 1`;;
	*)
	echo
	echo "ECHEC sur "$f" (pour une mauvaise raison)";;
    esac
done
echo

# les bons
echo -n "bons "
for f in typing/good/*.rs typing2/bad/*.rs typing2/good/*.rs exec/*.rs exec-fail/*.rs; do
    echo -n ".";
    max=`expr $max + 1`;
    compile --type-only $f;
    case $? in
	"1")
	echo
	echo "ECHEC sur "$f" (devrait reussir)";;
	"0") score=`expr $score + 1`;;
	*)
	echo
	echo "ECHEC sur "$f" (pour une mauvaise raison)";;
    esac
done
echo

percent=`expr 100 \* $score / $max`;

echo    "Typage  : $score/$max : $percent%";
}

partiea2 () {
echo
echo "Tests ambigüs partie 2"

for f in typing/ambiguous/*.rs; do
    echo -n "."
    if rustc $f -o a.out > /dev/null 2>&1; then
        compile --type-only $f;
        case $? in
        "0") ;;
        "1")
            echo
            echo "échec sur "$f" alors que rustc réussit";;
        *)
            echo
            echo "ECHEC (pour une mauvaise raison)";;
        esac
    else
        compile --type-only $f;
        case $? in
        "0")
            echo
            echo "réussite sur "$f" alors que rustc échoue";;
        "1") ;;
        *)
            echo
            echo "ECHEC (pour une mauvaise raison)";;
        esac
    fi


done

rm a.out
}

partie2b () {
echo
echo "Partie 2b"

score=0
max=0

# les mauvais
echo -n "mauvais "
for f in typing2/bad/*.rs; do
    echo -n ".";
    max=`expr $max + 1`;
    compile --no-asm $f;
    case $? in
	"0")
	echo
	echo "ECHEC sur "$f" (devrait échouer)";;
	"1") score=`expr $score + 1`;;
	*)
	echo
	echo "ECHEC sur "$f" (pour une mauvaise raison)";;
    esac
done
echo

# les bons
echo -n "bons "
for f in typing2/good/*.rs exec/*.rs exec-fail/*.rs; do
    echo -n ".";
    max=`expr $max + 1`;
    compile --no-asm $f;
    case $? in
	"1")
	echo
	echo "ECHEC sur "$f" (devrait reussir)";;
	"0") score=`expr $score + 1`;;
	*)
	echo
	echo "ECHEC sur "$f" (pour une mauvaise raison)";;
    esac
done
echo

percent=`expr 100 \* $score / $max`;

echo    "Typage  : $score/$max : $percent%";
}


# partie 3 : tests d'exécution

partie3 () {

score_comp=0
score_out=0
score_test=0
max=0

echo
echo "Partie 3"
echo "Execution normale"
echo "-----------------"

# timeout="why3-cpulimit 30 0 -h"
timeout=""

for f in exec/*.rs; do
    echo -n "."
    asm=exec/`basename $f .rs`.s
    rm -f $asm
    expected=exec/`basename $f .rs`.out
    max=`expr $max + 1`;
    if compile $f; then
	rm -f out
	score_comp=`expr $score_comp + 1`;
	if gcc $asm && ./a.out > out; then
	    score_out=`expr $score_out + 1`;
	    if cmp --quiet out $expected; then
		score_test=`expr $score_test + 1`;
	    else
		echo
		echo "ECHEC : mauvaise sortie pour $f"
	    fi
	else
		echo
		echo "ECHEC du code produit pour $f"
	fi
    else
	echo
	echo "ECHEC de la compilation sur $f (devrait réussir)"
    fi
done
echo

echo "Execution conduisant à un échec"
echo "-------------------------------"

for f in exec-fail/*.rs; do
    echo -n "."
    asm=exec-fail/`basename $f .rs`.s
    rm -f $asm
    max=`expr $max + 1`;
    if compile $f && gcc $asm; then
	score_comp=`expr $score_comp + 1`;
	if { ./a.out; } > /dev/null 2>&1; then
	    echo
	    echo "ECHEC : le code $f devrait échouer"
	else
	    score_test=`expr $score_test + 1`;
	    score_out=`expr $score_out + 1`;
	fi
    else
	echo
	echo "ECHEC de la compilation sur $f (devrait réussir)"
    fi
done

echo
percent=`expr 100 \* $score / $max`;

echo "Compilation:";
percent=`expr 100 \* $score_comp / $max`;
echo "Compilation : $score_comp/$max : $percent%";
percent=`expr 100 \* $score_out / $max`;
echo "Code produit : $score_out/$max : $percent%";
percent=`expr 100 \* $score_test / $max`;
echo "Comportement du code : $score_test/$max : $percent%";}

case $option in
    "-1" )
        partie1;;
    "-2" )
        partie2;;
    "-a2" )
        partiea2;;
    "-2b" )
        partie2b;;
    "-3" )
        partie3;;
    "-v1" )
	verbose=1;
	partie1;;
    "-v2" )
    	verbose=1;
        partie2;;
    "-v2b" )
    	verbose=1;
        partie2;;
    "-v3" )
    	verbose=1;
        partie3;;
    "-all" )
    	partie1;
    	partie2;
    	partie2b;
    	partie3;;
    "-rustc" )
        test_rustc;;
    * )
        echo "usage : $0 <option> <compilo>"
        echo "spécifier une option parmi : "
        echo "-1      : tester la syntaxe"
        echo "-2      : tester le typage"
        echo "-a2     : tester la partie ambigue du typage"
        echo "-2b     : tester le typage des ressources"
        echo "-3      : tester la production de code"
        echo "-v1     : tester la syntaxe (verbose)"
        echo "-v2     : tester le typage (verbose)"
        echo "-v3     : tester le typage des ressources (verbose)"
        echo "-all    : tout tester"
        echo "-rustc  : vérifier les tests sur le compilateur 'rustc'";;

esac
echo
