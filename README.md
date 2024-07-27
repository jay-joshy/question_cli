# README

## Purpose
Quick command line tool to both classify and/or answer allergy/immunology questions.

## Requirements
You required a .json file with questions formatted as such:
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

Open your command line. For Windows, this is "Command Prompt" or "PowerShell" app; for MacOS/Linux it is the "Terminal".
After question_cli, add either "classify" or "answer", followed by the PATH to the .json:
```zsh
question_cli <classify or answer> <insert_path_to_json>
```

Example:
```zsh
question_cli answer /home/josh/Documents/question_cli/questions.json
```
## Navigation
You should be able to navigate through the questions and classify each question. You are able to save your progress whenever you want.

y = classify as higher order;

n = classify as lower order;

f = move forward to the next question;

b = move backward to the previous question;

s = save the .json file with your edits;

q = quit and save the .json file with your edits;
