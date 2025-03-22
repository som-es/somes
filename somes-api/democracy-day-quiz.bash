curl -X POST "http://localhost:3000/add_quiz" \
     -H "Content-Type: application/json" \
     -d '{
          "title": "Politik und Demokratie - Democracy Day",
          "description": "Ein Quiz zu politischen Prinzipien und Demokratie.",
          "questions": [
            {
              "question": "Welches demokratische Prinzip beschreibt die Trennung von Gesetzgebung, Regierung und Rechtsprechung?",
              "answer1": "Föderalismus",
              "answer2": "Gewaltenteilung",
              "answer3": "Pluralismus",
              "answer4": "Subsidiarität",
              "correct_answer": 2
            },
            {
              "question": "Was bedeutet der Begriff „Populismus“ in der politischen Wissenschaft?",
              "answer1": "Eine Regierungsform, die sich durch eine besonders hohe Bürgerbeteiligung auszeichnet",
              "answer2": "Eine Strategie, die einfache Lösungen für komplexe Probleme verspricht und das „Volk“ gegen die „Eliten“ stellt",
              "answer3": "Eine Bewegung, die sich ausschließlich auf wirtschaftspolitische Themen konzentriert",
              "answer4": "Eine Form der direkten Demokratie",
              "correct_answer": 2
            },
            {
              "question": "Welches Kriterium muss erfüllt sein, damit eine Demokratie als „liberal“ gilt?",
              "answer1": "Es gibt jährlich Wahlen",
              "answer2": "Der Staat garantiert Grundrechte und schützt Minderheiten",
              "answer3": "Alle Entscheidungen werden durch das Volk direkt abgestimmt",
              "answer4": "Es gibt nur zwei große politische Parteien",
              "correct_answer": 2
            },
            {
              "question": "Welche Länder waren Gründungsmitglieder der Europäischen Gemeinschaft für Kohle und Stahl (EGKS)?",
              "answer1": "Deutschland, Frankreich, Italien, Belgien, Niederlande, Luxemburg",
              "answer2": "Deutschland, Frankreich, Spanien, Portugal, Österreich, Polen",
              "answer3": "Deutschland, Frankreich, Schweden, Norwegen, Dänemark, Finnland",
              "answer4": "Deutschland, Frankreich, Großbritannien, Irland, Griechenland, Ungarn",
              "correct_answer": 1
            },
            {
              "question": "Welches Element gehört NICHT zur repräsentativen Demokratie?",
              "answer1": "Freie Wahlen",
              "answer2": "Gewaltenteilung",
              "answer3": "Direkte Abstimmung über alle Gesetze durch das Volk",
              "answer4": "Grundrechte",
              "correct_answer": 3
            },
            {
              "question": "Welche Bedingungen müssen für ein erfolgreiches Misstrauensvotum im Nationalrat erfüllt sein?",
              "answer1": "Es muss eine Zweidrittelmehrheit der Abgeordneten zustimmen.",
              "answer2": "Die Mehrheit der anwesenden Abgeordneten muss zustimmen, wobei mindestens die Hälfte aller Mitglieder anwesend sein muss.",
              "answer3": "Es muss von mindestens der Hälfte aller Abgeordneten beantragt werden.",
              "answer4": "Der Bundespräsident entscheidet allein über den Erfolg des Misstrauensvotums.",
              "correct_answer": 2
            },
            {
              "question": "Was möchte die Initiative „Österreich der runden und eckigen Tische“ bewirken?",
              "answer1": "Gesellschaftliche Spaltung und Unterschiede mit Dialog reduzieren",
              "answer2": "Konflikte durch mehr direkte Konfrontation und Proteste lösen",
              "answer3": "Eine einheitliche politische Meinung in der Gesellschaft etablieren",
              "answer4": "Politische Entscheidungen ausschließlich Experten überlassen, um gesellschaftliche Polarisierung zu vermeiden",
              "correct_answer": 1
            },
            {
              "question": "Welches Prinzip gehört NICHT zu den notwendigen politischen Haltungen, um Grenzen zu überwinden?",
              "answer1": "Zuhören",
              "answer2": "Verantwortung übernehmen",
              "answer3": "Selbstbezogenheit",
              "answer4": "Vergebung",
              "correct_answer": 3
            }
          ]
        }'
