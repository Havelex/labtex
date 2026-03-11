% LTeX: language=de-DE
% ============================================================
%  protokoll.cls
%  Klasse für Protokolle „Einführung in die physikalischen
%  Messmethoden" (TU Graz / Uni Graz / NAWI Graz)
%
%  Verwendung:  \documentclass{protokoll}
% ============================================================
\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{protokoll}[2025/03/10 Physik-Praktikum Protokoll]

\LoadClass[12pt, a4paper]{article}

% ── Pakete ───────────────────────────────────────────────────
\RequirePackage[utf8]{inputenc}
\RequirePackage[T1]{fontenc}
\RequirePackage{newpxtext}
\RequirePackage{newpxmath}
\RequirePackage{helvet}
\RequirePackage[ngerman]{babel}
\RequirePackage{geometry}
\RequirePackage{graphicx}
\RequirePackage{amsmath}
\RequirePackage{booktabs}
\RequirePackage{array}
\RequirePackage{caption}
\RequirePackage{fancyhdr}
\RequirePackage{hyperref}
\RequirePackage{parskip}
\RequirePackage{microtype}
\RequirePackage{lastpage}
\RequirePackage{xparse}
\RequirePackage{bookmark}
\RequirePackage[normalem]{ulem}
\RequirePackage{titlesec}
\RequirePackage{siunitx}
\RequirePackage{float}
\RequirePackage{csquotes}
\RequirePackage[
  backend=biber,
  style=numeric,
  sorting=none,
  urldate=iso,
  date=iso,
  seconds=true,
]{biblatex}

\geometry{top=2.5cm, bottom=2.5cm, left=2.5cm, right=2.5cm, headheight=32pt}
\titleformat{\section}{\sffamily\large\bfseries}{\thesection}{0.5em}{}
\sisetup{
  locale            = DE,
  group-separator   = {.},      % 1.000.000
  output-decimal-marker = {,},  % 1,5 instead of 1.5
  group-digits      = integer,  % only group before decimal
  group-minimum-digits = 4,     % 1000 not grouped, 10.000 is
  exponent-product  = \cdot,    % 1,5 · 10³ (German style, not ×)
  output-exponent-marker = \text{e}, % alternative: e-notation
  inter-unit-product = \cdot,   % kg·m·s⁻² (German uses · not space)
  per-mode          = symbol,   % m/s instead of m s⁻¹ (adjust to taste)
  detect-all,                   % match surrounding font weight/shape
  separate-uncertainty=true
}
\setlength{\extrarowheight}{4pt}

% ============================================================
%  Metadaten-Befehle
% ============================================================
\newcommand{\ueNum}[1]{\def\@ueNum{#1}}
\newcommand{\group}[1]{\def\@group{#1}}
\newcommand{\ueTitle}[1]{\def\@ueTitle{#1}}
\newcommand{\supervisor}[1]{\def\@supervisor{#1}}
\newcommand{\ueDate}[1]{\def\@ueDate{#1}}
\newcommand{\semester}[1]{\def\@semester{#1}}

% \authorOne{Vorname}{Nachname}{Matrikelnummer}
\newcommand{\authorOne}[3]{%
  \def\@authorOneFirst{#1}%
  \def\@authorOneLast{#2}%
  \def\@authorOneName{#1 #2}%
  \def\@authorOneMatrnr{#3}%
}
% \authorTwo{Vorname}{Nachname}{Matrikelnummer}
\NewDocumentCommand{\authorTwo}{mmm}{%
  \def\@authorTwoFirst{#1}%
  \def\@authorTwoLast{#2}%
  \def\@authorTwoName{#1 #2}%
  \def\@authorTwoMatrnr{#3}%
  \def\@hasAuthorTwo{true}%
}

% Logo-Dateinamen (ohne Endung, mit relativem Pfad)
\newcommand{\logoTUGraz}[1]{\def\@logoTUGraz{#1}}
\newcommand{\logoNAWIGraz}[1]{\def\@logoNAWIGraz{#1}}
\newcommand{\logoUniGraz}[1]{\def\@logoUniGraz{#1}}

% Standardwerte
\def\@ueNum{?}
\def\@group{?}
\def\@ueTitle{?}
\def\@supervisor{?}
\def\@ueDate{?}
\def\@semester{?}
\def\@authorOneFirst{?}
\def\@authorOneLast{?}
\def\@authorOneName{?}
\def\@authorOneMatrnr{?}
\def\@authorTwoFirst{?}
\def\@authorTwoLast{?}
\def\@authorTwoName{?}
\def\@authorTwoMatrnr{?}
\def\@hasAuthorTwo{false}
\def\@logoTUGraz{logos/logo_tugraz}
\def\@logoNAWIGraz{logos/logo_nawigraz}
\def\@logoUniGraz{logos/logo_unigraz}

\ExplSyntaxOn
\NewDocumentCommand{\initial}{m}{%
  \exp_args:No \str_head:n { #1 }%
}
\ExplSyntaxOff

% ============================================================
%  Kopf-/Fußzeile
% ============================================================
\pagestyle{fancy}
\fancyhf{}
\renewcommand{\headrulewidth}{0.4pt}
\def\@True{true}

\AtBeginDocument{%
  \lhead{Gruppe \@group,
    \initial\@authorOneFirst.\ \@authorOneLast
    \ifx\@hasAuthorTwo\@True{} \& \initial\@authorTwoFirst.\ \@authorTwoLast\fi}%
    \rhead{\parbox[b]{5cm}{\raggedright \@ueTitle,} \@ueDate}%
  \cfoot{\thepage/\pageref{LastPage}}%
}

\fancypagestyle{plain}{%
  \fancyhf{}%
  \renewcommand{\headrulewidth}{0pt}%
}

% ============================================================
%  Titelseite
% ============================================================
\renewcommand{\maketitle}{%
  \begin{titlepage}
    \centering
    \begin{minipage}[c]{0.40\textwidth}
      \includegraphics[width=5cm]{\@logoTUGraz}%
    \end{minipage}%
    \hfill
    \begin{minipage}[c]{0.40\textwidth}
      \raggedleft
      \includegraphics[width=3cm]{\@logoUniGraz}%
    \end{minipage}

    \vspace{0.4cm}

    \begin{center}
      \includegraphics[width=8cm]{\@logoNAWIGraz}%
    \end{center}

    \vspace{1.5cm}
    {\large \uline{Institut für Experimentalphysik der}}\\[0.2cm]
    {\large \uline{Technischen Universität Graz}}\\[0.4cm]
    {\large \&}\\[0.4cm]
    {\large \uline{Institut für Physik der Universität Graz}}
    \vspace{1.5cm}

    {\fontsize{28}{34}\selectfont\bfseries
      \MakeUppercase{Einführung in die}\\[0.2cm]
      \MakeUppercase{Physikalischen}\\[0.2cm]
      \MakeUppercase{Messmethoden}}

    \vspace{2cm}

    \begin{flushleft}
      Übungsnummer: \@ueNum\\[0.3cm]
      Gruppennummer: \@group\\[0.3cm]
      Übungstitel: \@ueTitle\\[0.6cm]
      Betreuer:in: \@supervisor\\[0.6cm]
      \ifx\@hasAuthorTwo\@True
        \begin{tabular}{@{}ll}
          Name 1: \@authorOneName & Name 2: \@authorTwoName \\
          Mat.\ Nr.: \@authorOneMatrnr & Mat.\ Nr.: \@authorTwoMatrnr
        \end{tabular}
      \else
        Name: \@authorOneName \quad Mat.\ Nr.: \@authorOneMatrnr
      \fi
    \end{flushleft}

    \vfill
    \begin{flushleft}
      Datum der Übung: \@ueDate \hfill \@semester
    \end{flushleft}
  \end{titlepage}
  \setcounter{page}{2}%
}
