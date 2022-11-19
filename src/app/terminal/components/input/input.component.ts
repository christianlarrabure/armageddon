import { Component, Input, OnInit } from '@angular/core';
import { TelnetService } from '../../../shared/services/telnet.service';

@Component({
  selector: 'app-input',
  templateUrl: './input.component.html',
  styleUrls: ['./input.component.scss'],
})
export class InputComponent implements OnInit {
  inputHistory: string[] = [];
  position: number = 0;
  input: string = '';

  constructor(private telnetService: TelnetService) {}

  ngOnInit(): void {}

  submitInput(event: Event) {
    this.telnetService.send(this.input);
    const historyLength = this.inputHistory.length;
    if (this.input.length === 0) {
      return;
    }
    if (this.input === this.inputHistory[historyLength]) {
      return;
    }
    this.inputHistory.push(this.input);
    this.position = this.inputHistory.length;
    this.input = '';
  }

  keydownEnter(event: Event) {
    event.preventDefault();
  }

  goHistory(steps: number) {
    const historyLength = this.inputHistory.length;
    if (this.position === historyLength && steps > 0) {
      return;
    }
    if (this.position === 0 && steps < 0) {
      return;
    }
    this.position = this.position + steps;
    this.input = this.inputHistory[this.position];
  }

  autocomplete(event: Event) {
    event.preventDefault();
    if (this.input.length === 0) {
      return;
    }

    const bestCandidate = this.inputHistory.filter((value: String) => {
      return value.includes(this.input, 0);
    });

    if (bestCandidate.length === 0) {
      return;
    }

    this.input = bestCandidate[bestCandidate.length - 1];
  }
}
