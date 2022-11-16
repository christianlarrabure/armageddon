import { Component, AfterViewInit, ViewChild, Input } from '@angular/core';
import { NgTerminal } from 'ng-terminal';
import { TelnetService } from '../../../shared/services/telnet.service';

@Component({
  selector: 'app-terminal',
  templateUrl: './terminal.component.html',
  styleUrls: ['./terminal.component.scss'],
})
export class TerminalComponent implements AfterViewInit {
  @ViewChild('term', { static: false }) term!: NgTerminal;
  @Input() background?: string;
  @Input() foreground?: string;
  @Input() terminalWidth?: number;
  constructor(private telnetService: TelnetService) {}

  ngAfterViewInit() {
    this.telnetService.init();

    this.term.setXtermOptions({
      theme: {
        background: this.background ?? '#111827',
        cursor: this.background ?? '#111827',
        foreground: this.foreground ?? '#FFFFFF',
      },
      convertEol: true,
      minimumContrastRatio: 7,
    });

    this.telnetService.messages$.subscribe((message) => {
      this.term.underlying.write(message);
    });
  }
}
