export class Greeting {
  //Get the Day from Client Machine
  date = new Date();
  day = this.date.getDay();
  hour = this.date.getHours();

  itsNewMonth: boolean = this.date.getDate() == 1;
  itsChristmas: boolean =
    this.date.getMonth() == 11 && this.date.getDate() == 25;
  itsValentine: boolean =
    this.date.getMonth() == 1 && this.date.getDate() == 14;

  private message: string = "";

  private possibleGreetings = {
    monday: ["Hello Monday!", "Monday is here!", "Another fresh start"],
    tuesday: ["Hello"],
    wednesday: ["Welcome Back"],
    thursday: [
      "Throwback Thursday😋",
      "Time for throwback",
      "let's do some throwback",
      "Hectic week, huh? ",
    ],
    friday: [
      "#TGIF",
      "Weekend is here!",
      "Have a nice weekend",
      "How has the week been?",
      "Thank God It's Friday",
      "It's weekend",
      "Hello",
      "Weekend is near",
    ],
    saturday: [
      "You deserve some break",
      "Take timeout, enjoy",
      "Hangout! It's saturday",
      "Still indoor? Hangout!",
      "You deserve a great time",
      "How about some treats?",
    ],
    sunday: [
      "Happy New Week!",
      "Happy Sunday!",
      "It's a new week💃💃",
      "New week wishes",
    ],
    morning: [
      "Blessed morning",
      "Good Morning",
      "Trust you slept well",
      "Beautiful morning",
      "A new day",
      "Have a fruitful day",
    ],
    day_break: [
      "How is your day going",
      "How has the day being",
      "A fresh morning, huh?",
      "How's the weather?",
      "Howdy?",
    ],
    afternoon: [
      "Good afternoon!",
      "How's the weather outside",
      "How is your day going",
      "How has work been?",
      "How has your day been?",
      "How is your day going",
      "How has the day been",
      "How's the weather?",
      "Howdy?",
    ],
    evening: ["Good evening", "How was your day?", "How did your day go?"],
    midNight: [
      "You up so late?",
      "Surprised to see you up so late",
      "Up so early, huh?",
      "Up so early?",
      "Working Lat?",
      "Hi there!",
      "Not sleeping?",
    ],
    newMonth: [
      "Happy new month!",
      "Blessed new Month",
      "Have a Happy new month!",
      "It's a new month!",
    ],
    newYear: ["Happy new year", "Happy Holidays"],
    christmas: ["Merry Christmas", "Season greetings"],
    valentine: ["It's Valentine", "Happy Valentine"],
    other: ["Holla!", "Hello!", "Welcome", "Howdy doody!", "Ciao!"],
  };

  // an Util Function to Get Random Items in This Case, Greeting Options
  fetchRandomElementFromArray(array: Array<string>) {
    const index = Math.round(Math.random() * (array.length - 1));
    return array[index];
  }

  constructor() {
    if (this.itsChristmas) {
      this.message = this.fetchRandomElementFromArray(
        this.possibleGreetings.christmas
      );
    } else if (this.itsValentine) {
      this.message = this.fetchRandomElementFromArray(
        this.possibleGreetings.valentine
      );
    } else if (this.itsNewMonth) {
      this.message = this.fetchRandomElementFromArray(
        this.possibleGreetings.newMonth
      );
    } else if (this.day == 0 || this.day == 4 || this.day == 6) {
      switch (this.day) {
        //If Sunday
        case 0:
          this.message = this.fetchRandomElementFromArray(
            this.possibleGreetings.sunday
          );
          break;
        // case 1:
        //   //If Monday
        //   this.message = this.fetchRandomElementFromArray(
        //     this.possibleGreetings.monday
        //   );
        //   break;
        //If Thursday
        case 4:
          this.message = this.fetchRandomElementFromArray(
            this.possibleGreetings.thursday
          );
          break;
        //If Friday
        // case 5:
        //   this.message = this.fetchRandomElementFromArray(
        //     this.possibleGreetings.friday
        //   );
        //   break;
        //If Saturday
        case 6:
          this.message = this.fetchRandomElementFromArray(
            this.possibleGreetings.saturday
          );
          break;
        default:
          this.message = "";
          break;
      }
    } else {
      switch (true) {
        //If between 6:00am & 8:00am
        case this.hour >= 6 && this.hour <= 8:
          this.message = this.fetchRandomElementFromArray(
            this.possibleGreetings.morning
          );
          break;
        //if between 8:00AM & 10:00AM
        case this.hour >= 8 && this.hour <= 10:
          this.message = this.fetchRandomElementFromArray(
            this.possibleGreetings.day_break
          );
          break;
        //if afternoon 12:00PM - 3:00PM
        case this.hour >= 12 && this.hour <= 15:
          this.message = this.fetchRandomElementFromArray(
            this.possibleGreetings.afternoon
          );
          break;
        //if evening : 4:00PM - 8:00PM
        case this.hour >= 16 && this.hour <= 20:
          this.message = this.fetchRandomElementFromArray(
            this.possibleGreetings.evening
          );
          break;
        //if midnight between 1:00AM - 3:00AM
        case this.hour >= 1 && this.hour <= 3:
          this.message = this.fetchRandomElementFromArray(
            this.possibleGreetings.midNight
          );
          break;
      }
    }
  }

  get msg() {
    return this.message;
  }
}
