# README

## Purpose
Quick internal command line tool to both classify and/or answer questions.

## Requirements
You require a .json file with questions formatted as such:
```json
[
  {
    "question": "Infection with X drug makes you more likely to have infection from the following?",
    "options": [
      "Histoplasma",
      "Escherichia coli",
      "Listeria monocytogenes",
      "HIV",
      "Streptococcus spp."
    ],
    "answer": "HIV"
  },
  {
    "question": "A 43-year-old woman ...  Which one of the following tests is most likely to be diagnostic in this case?",
    "options": [
      "CXR",
      "CBC",
      "ALT",
      "CT Chest",
      "ANA"
    ],
    "answer": "ANA"
  }
]
```

## How to use

Open your command line and run the tool using the format below. For Windows, this is the "Command Prompt" or "PowerShell" app; for MacOS/Linux it is the "Terminal".
```zsh
question_cli <classify or answer> <path_to_json>
```
Example:
```zsh
question_cli answer /home/josh/Documents/question_cli/questions.json
```

Once running the tool, instructions are provided on how to navigate through each question.
In brief, type <y> or <n> to classify questions as higher or lower order. If answering, simply type the option #.
Navigate through questions with your arrow keys.

Enjoy!
