/* Reusable retrieval-practice quiz component for chess-raw lessons.
   Usage in a lesson:

     <div class="quiz" data-answer="1">
       <p class="quiz-q">Question text?</p>
       <button>Option A</button>
       <button>Option B</button>
       <p class="quiz-why">Explanation shown after answering.</p>
     </div>
     <script src="../assets/quiz.js"></script>

   data-answer is the zero-based index of the correct button.
   Feedback is immediate and automatic. Keep option text the same length. */

(function () {
  const style = document.createElement("style");
  style.textContent = `
    .quiz { border: 1px solid var(--rule); border-radius: 5px; padding: 1.2rem 1.4rem; margin: 2rem 0; }
    .quiz-q { margin: 0 0 .9rem; font-weight: 600; font-size: 1rem; }
    .quiz button {
      display: block; width: 100%; text-align: left; margin: .4rem 0;
      padding: .6rem .8rem; font: inherit; font-size: .95rem;
      background: var(--code-bg); color: var(--ink);
      border: 1px solid var(--rule); border-radius: 4px; cursor: pointer;
    }
    .quiz button:hover:not(:disabled) { border-color: var(--accent); }
    .quiz button:disabled { cursor: default; opacity: .95; }
    .quiz button.right { border-color: var(--good); box-shadow: inset 3px 0 0 var(--good); }
    .quiz button.wrong { border-color: var(--bad); box-shadow: inset 3px 0 0 var(--bad); }
    .quiz-why { display: none; margin: 1rem 0 0; font-size: .93rem; color: var(--ink-soft); }
    .quiz.done .quiz-why { display: block; }
    .quiz-score { font-family: ui-monospace, monospace; font-size: .8rem; color: var(--ink-soft); margin-top: 2rem; }
  `;
  document.head.appendChild(style);

  const quizzes = document.querySelectorAll(".quiz");
  let asked = 0;
  let right = 0;

  quizzes.forEach((quiz) => {
    const correct = Number(quiz.dataset.answer);
    const buttons = [...quiz.querySelectorAll("button")];

    buttons.forEach((btn, i) => {
      btn.addEventListener("click", () => {
        if (quiz.classList.contains("done")) return;
        quiz.classList.add("done");
        asked += 1;
        if (i === correct) right += 1;
        buttons.forEach((b, j) => {
          b.disabled = true;
          if (j === correct) b.classList.add("right");
          else if (j === i) b.classList.add("wrong");
        });
        updateScore();
      });
    });
  });

  let scoreEl = null;
  function updateScore() {
    if (!quizzes.length) return;
    if (!scoreEl) {
      scoreEl = document.createElement("p");
      scoreEl.className = "quiz-score";
      quizzes[quizzes.length - 1].after(scoreEl);
    }
    scoreEl.textContent = `Recall: ${right} / ${asked} answered, ${quizzes.length} total.`;
  }
})();
