with open('answers.txt') as answers_file:
    answered_questions = 0

    questions = [0 for _ in range(26)]

    for line in answers_file:
        if line != "\n":
            for char in line.strip():
                questions[ord(char) - 97] = 1
        else:
            answered_questions += sum(questions)
            questions = [0 for _ in range(26)]

    print(answered_questions + sum(questions))
