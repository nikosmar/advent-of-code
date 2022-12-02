def evaluate_group(group_questions, group_size):
    group_answered_questions = 0

    for i in range(len(group_questions)):
        if group_questions[i] == group_size:
            group_answered_questions += 1

        group_questions[i] = 0

    return group_answered_questions


with open('answers.txt') as answers_file:
    answered_questions = 0

    questions = [0 for _ in range(26)]
    people_in_group = 0

    for line in answers_file:
        if line != "\n":
            for char in line.strip():
                questions[ord(char) - 97] += 1

            people_in_group += 1
        else:
            answered_questions += evaluate_group(questions, people_in_group)
            people_in_group = 0

    print(answered_questions + evaluate_group(questions, people_in_group))
