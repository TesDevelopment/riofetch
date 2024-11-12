<h3 align="center"><img src="./assets/riofetch.png" alt="logo" height="150px"></h3>
<p align="center">A command-line roborio information tool written in rust </p>

***

### Where is the riofetch config?
Your riofetch config can be found at `~/riofetch.json` across all operating systems/

### What do all these symbols mean?
While it may look daunting initially, riofetch formatting is made with ease of use in mind, here a guide on all the formatting rules.


## Formats

#### What is a format?
Formats are peices of text that are replaced at runtime. A format can be identified by its ${format} notation.

#### What kind of formats are there?
There are 5 seperate formatting options, most formatting options have a subformat identified by an Uppercase letter.

## ${f[?]} - Text formatting
    ${fB} - Bold
    ${fI} - Italic
    ${fU} - Underline
    ${fS} - Strikethrough / Crossout

## ${c[?]} - Color formatting
    ${cW} - White
    ${cR} - Red
    ${cB} - Blue
    ${cG} - Green
    ${c?} - Random

## ${b[?]} - Boolean Check
The boolean check is rarely used, and functions differently than other formats. The boolean check should be the first format on a line and will remove the line of it doesnt't evaluate to true.

    ${bP} - Detected Pathplanner
    ${bN} - No autos detected
    ${bC} - Detected Choreo (WIP)

## ${r[?]} - Repeat character
The repeat format is used for seperators, whatever character used will be repeated to match the length of the previous line.

    ${r-} -> -----------------

## ${i[?]} - Information
Arguably the most important format, this allows you to place actual information into riofetch.

    ${iL} - Programming Language
    ${iT} - Rio Type / Version (1 || 2)
    ${iP} - Pathplanner autos (Seperated by pipe)
    ${iH} - Code Hash
    ${iM} - Current memory usage
    ${iN} - Device Name
    ... More WIP
