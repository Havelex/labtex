# LabTeX

This project contains everything you need to write protocols for EPM.
Because I have better things to do, for now, the only template language is
German.

> [!NOTE]
> This template has only been tested on Linux. It should work on Mac and Windows
> too, though installation paths and commands may vary.

## Installation

### Linux

Run the following command in your terminal from this directory:

```bash
mkdir -p ~/texmf/tex/latex/
cp -r protokoll ~/texmf/tex/latex/
```

Then just copy the `protokoll.tex` file to your project and start writing.

### Windows

Run the following command in **CMD** from this directory:

```bash
mkdir "%APPDATA%\MiKTeX\tex\latex\protokoll"
xcopy protokoll "%APPDATA%\MiKTeX\tex\latex\protokoll\" /E /I
```

_OR_

run the following command in your **PowerShell** from this directory:

```bash
New-Item -ItemType Directory -Force -Path "$env:APPDATA\MiKTeX\tex\latex\protokoll"
Copy-Item protokoll -Destination "$env:APPDATA\MiKTeX\tex\latex\protokoll\" -Recurse
```

### Mac

Run the following command in your terminal from this directory:

```bash
mkdir -p ~/Library/texmf/tex/latex/
cp -r protokoll ~/Library/texmf/tex/latex/
```

Then just copy the `protokoll.tex` file to your project and start writing.

### Overleaf (Universal)

To install this into overleaf, simply upload the contents of the `src/` folder
into your project. Delete your `main.txt` (or move its contents into
`protokoll.tex`) and move the contents of `protokoll` into the root directory
(but keep the logos inside `logos/`).

Now make `protokoll.tex` to be your new `main` by pressing the three dots and
selecting the corresponding option.

## Usage

After installation, simply copy the `protokoll.tex` file to the root of your
project and start writing. Edit the metadata section to fit the title page and
the footer / header to your personal values.

### Useful Features

This template class already import several packages that provide countless
useful features. Here are some of the features from packages and built into
LaTeX to make your protocol look prettier.

But for now I only talk about `siunitx`, because it's past midnight and I am
tired.

#### siunitx

The package `siunitx` provides very helpful features for displaying physical
quantities, numbers and units.

The template has set all options to format everything according to german
standards.

Here are some examples for some of the functions,
however there are many more of both functions and usecases:

##### Numbers

Parse numbers via the `\num` command:

```tex
\num{3}         % 3
\num{1.2}       % 1,2
\num{4e6}       % 1.000.000
\num{1+5j}      % 1 + 5j
\num{-1}        % negative
\num{+2}        % explicit positiv
\num{+-3}       % ± 3
\num{10.7(5)}   % 10,7 ± 0,5
\num{10.7(2.5)} % 10,7 ± 2,5
```

##### Units

Use predefined SI units or define custom ones via the `\unit` command:

```tex
\unit\ohm           % Ω
\unit\degreeCelsius % °C
\unit{p.m.}         % p.m
```

> [!NOTE]
> To use units outside of equations you should use their text variant.
> e.g.: `\unit\ohm` => `\textohm`
> However, not all units have a text variant.
> e.g.: `\unit\degreeCelsius` => ~`\textdegreeCelsius`~
> In that case you must get creative:
> `\unit\degreeCelsius` => `\textdegree C`

##### Quantities

Write formatted physical quantities via the `\qty` command (first argument =
value, second argument = unit):

```tex
\qty{123}{\ohm}        % 123 Ω
\qty{+-5}{\percent}    % ±5 %
\qty{420.69(4)}{\volt} % (420,69 ± 0,04) V
```

> [!NOTE]
> It is valid to omit the commands `\num` and `\unit` as seen in the examples
> above.

##### Tables

For this use the normal `table` and `tabular` environments and the column
specifier `S` to declare a column to be `siunitx` compatible cells:

```tex
\begin{table}[H]
    \centering
    \caption{Vergleich: Herstellerangaben vs Messergebnis}
    \label{tab:comparison}
    \begin{tabular}{| c | S | S |}
        \hline
        Größe & {Herstellerwert / \textohm} & {Messergebnis / \textohm} \\
        \hline\hline
        $I$ & 470(5) & 468.5(2.3) \\
        \hline
    \end{tabular}
\end{table}
```

> [!NOTE]
> The specifier `S` allows you to simply write plaintext numbers into the cells
> and `siunitx` will parse them.

> Furthermore, the first row is always text, regardless of the modifier, since
> it acts as headers.
