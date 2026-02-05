import type { ComponentChildren } from "preact";
import cls from "./feedback.module.css";

export function FeedbackPage(): ComponentChildren {
  return (
    <div className={cls.container}>
      <h1>How's my posting?</h1>
      <p>
        We here at Lexi value your input. Our staff is standing by 24/7 to read
        and consider your feedback. Submit your thoughts now!
      </p>
      <form className={cls.form}>
        <div>
          <label for="author">
            Your name <i>(optional)</i>
          </label>
          <input name="author" type="text" placeholder="Anonymous" />
        </div>
        <div>
          <label for="message">Your feedback</label>
          <textarea name="message" rows={5} />
        </div>
      </form>
      <div className={cls.submit}>Submit</div>
    </div>
  );
}
