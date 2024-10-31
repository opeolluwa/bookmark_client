export class Greeting {
  // Get the current date and time
  private date: Date = new Date();
  private day: number = this.date.getDay();
  private hour: number = this.date.getHours();

  private isNewMonth: boolean = this.date.getDate() === 1;
  private isChristmas: boolean =
    this.date.getMonth() === 11 && this.date.getDate() === 25;
  private isValentine: boolean =
    this.date.getMonth() === 1 && this.date.getDate() === 14;

  private message: string = "";

  private possibleGreetings: { [key: string]: string[] } = {
    monday: ["Hello Monday!", "Monday is here!", "Another fresh start"],
    tuesday: ["Hello!"],
    wednesday: ["Welcome Back!"],
    thursday: [
      "Throwback Thursday😋",
      "Time for throwback",
      "Let's do some throwback",
      "Hectic week, huh?",
    ],
    friday: [
      "#TGIF",
      "Weekend is here!",
      "Have a nice weekend",
      "How has the week been?",
      "Thank God It's Friday",
    ],
    saturday: [
      "You deserve a break!",
      "Take time out, enjoy",
      "Hangout! It's Saturday",
      "Still indoors? Hangout!",
      "You deserve a great time!",
    ],
    sunday: [
      "Happy New Week!",
      "Happy Sunday!",
      "It's a new week💃💃",
      "New week wishes",
    ],
    morning: [
      "Blessed morning",
      "Good morning",
      "Trust you slept well",
      "Beautiful morning",
      "A new day",
      "Have a fruitful day",
    ],
    day_break: [
      "How is your day going?",
      "How has the day been?",
      "A fresh morning, huh?",
      "How's the weather?",
      "Howdy?",
    ],
    afternoon: [
      "Good afternoon!",
      "How's the weather outside?",
      "How is your day going?",
      "How has work been?",
      "How has your day been?",
    ],
    evening: ["Good evening!", "How was your day?", "How did your day go?"],
    midnight: [
      "You up so late?",
      "Surprised to see you up so late",
      "Up so early, huh?",
      "Working late?",
      "Hi there!",
      "Not sleeping?",
    ],
    newMonth: [
      "Happy new month!",
      "Blessed new month!",
      "Have a happy new month!",
      "It's a new month!",
    ],
    newYear: ["Happy new year!", "Happy Holidays!"],
    christmas: ["Merry Christmas!", "Season's greetings!"],
    valentine: ["It's Valentine!", "Happy Valentine!"],
    other: ["Holla!", "Hello!", "Welcome!", "Howdy doody!", "Ciao!"],
  };

  constructor() {
    this.setGreetingMessage();
  }

  private fetchRandomElementFromArray(array: string[]): string {
    const index = Math.floor(Math.random() * array.length);
    return array[index];
  }

  private setGreetingMessage(): void {
    if (this.isChristmas) {
      this.message = this.fetchRandomElementFromArray(
        this.possibleGreetings.christmas
      );
    } else if (this.isValentine) {
      this.message = this.fetchRandomElementFromArray(
        this.possibleGreetings.valentine
      );
    } else if (this.isNewMonth) {
      this.message = this.fetchRandomElementFromArray(
        this.possibleGreetings.newMonth
      );
    } else {
      this.setDayBasedGreeting();
    }
  }

  private setDayBasedGreeting(): void {
    switch (this.day) {
      case 0: // Sunday
        this.message = this.fetchRandomElementFromArray(
          this.possibleGreetings.sunday
        );
        break;
      case 1: // Monday
        this.message = this.fetchRandomElementFromArray(
          this.possibleGreetings.monday
        );
        break;
      case 4: // Thursday
        this.message = this.fetchRandomElementFromArray(
          this.possibleGreetings.thursday
        );
        break;
      case 6: // Saturday
        this.message = this.fetchRandomElementFromArray(
          this.possibleGreetings.saturday
        );
        break;
      default:
        this.setTimeBasedGreeting();
        break;
    }
  }

  private setTimeBasedGreeting(): void {
    if (this.hour >= 6 && this.hour < 10) {
      this.message = this.fetchRandomElementFromArray(
        this.possibleGreetings.morning
      );
    } else if (this.hour >= 10 && this.hour < 12) {
      this.message = this.fetchRandomElementFromArray(
        this.possibleGreetings.day_break
      );
    } else if (this.hour >= 12 && this.hour < 16) {
      this.message = this.fetchRandomElementFromArray(
        this.possibleGreetings.afternoon
      );
    } else if (this.hour >= 16 && this.hour < 20) {
      this.message = this.fetchRandomElementFromArray(
        this.possibleGreetings.evening
      );
    } else if (this.hour >= 0 && this.hour < 4) {
      this.message = this.fetchRandomElementFromArray(
        this.possibleGreetings.midnight
      );
    } else {
      this.message = this.fetchRandomElementFromArray(
        this.possibleGreetings.other
      );
    }
  }

  public get msg(): string {
    return this.message;
  }
}
