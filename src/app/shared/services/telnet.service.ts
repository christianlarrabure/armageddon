import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { appWindow } from '@tauri-apps/api/window';
import { Event } from '@tauri-apps/api/event';
import { Subject } from 'rxjs';
import IPromptPayload from '../interfaces/prompt-payload.interface';

@Injectable({
  providedIn: 'root',
})
export class TelnetService {
  private _messages$ = new Subject<Uint8Array>();
  messages$ = this._messages$.asObservable();

  private _prompt$ = new Subject<IPromptPayload>();
  prompt$ = this._prompt$.asObservable();

  constructor() {
    appWindow.listen('telnet-message', (event: Event<Uint8Array>) => {
      this._messages$.next(event.payload);
    });

    appWindow.listen('armageddon-prompt', (event: Event<IPromptPayload>) => {
      this._prompt$.next(event.payload);
    });
  }

  init() {
    invoke('init');
  }

  connect() {
    invoke('connect').then((response: any) => {
      console.log(response);
    });
  }

  send(input: string) {
    if (input.toLowerCase() === '#connect') {
      this.connect();
      return;
    }
    if (input.toLowerCase() === '#prompt') {
      invoke('set_prompt');
      return;
    }
    if (input[0] === '#') {
      const inputargs = input.split(' ');
      const verb = inputargs[0].slice(1).toLowerCase();
      const args = inputargs.slice(1);
      invoke(`pcommand_${verb}`, { args: args.join(' ') });
      return;
    }
    invoke('send', { input: input });
  }
}
