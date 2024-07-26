# README

## Purpose
Quick command line tool to classify allergy/immunology questions.

## How to use

Open your command line. For Windows, this is "Command Prompt" or "PowerShell" app; for MacOS/Linux it is the "Terminal".
Enter the PATH to the .json with the questions after question_cli:
```zsh
question_cli <insert_path_to_json>
```

Example:
```zsh
question_cli /home/josh/Documents/question_cli/questions.json
```
## Navigation
You should be able to navigate through the questions and classify each question.

y = classify as higher order;
n = classify as lower order;
f = move forward to the next question;
b = move backward to the previous question;
s = save the .json file with your edits;
q = quit and save the .json file with your edits;
