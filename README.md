# README

## Purpose
Quick command line tool to classify allergy/immunology questions.

## How to use

Open your command line. For Windows, this is "Command Prompt" or "PowerShell" app; for MacOS/Linux it is the "Terminal".
Enter in the following format, where the second arguement is the PATH to the .json file with the questions.

> to find the PATH to any file, see this how-to if unclear: https://www.sony.com/electronics/support/articles/00015251
> (MacOS and Windows are fairly similar)

```zsh
question_cli /home/josh/Documents/question_cli/questions.json
```

## Navigation
You should be able to navigate through the questions and classify each question.
y = classify as higher order
n = classify as lower order
f = move forward to the next question
b = move backward to the previous question
s = save the .json file with your edits
q = quit and save the .json file with your edits

