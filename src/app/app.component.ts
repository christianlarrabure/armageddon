import { Component, AfterViewInit } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import ISettings from './shared/interfaces/settings.interface';
import {
  IStatBarSetting,
  IPlayerInfoSettings,
} from './terminal/interfaces/stat-bar.interface';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements AfterViewInit {
  title = 'arm-client';

  settings: ISettings = {};

  ngAfterViewInit() {
    invoke('get_config', { field: 'terminalBackground' }).then((value) => {
      if (typeof value === 'string') {
        this.settings.terminalBackground = `#${value}`;
        console.log(
          `Terminal background set to ${this.settings.terminalBackground}.`
        );
      }
    });
    invoke('get_config', { field: 'terminalForeground' }).then((value) => {
      if (typeof value === 'string') {
        this.settings.terminalForeground = `#${value}`;
        console.log(
          `Terminal foreground set to ${this.settings.terminalForeground}.`
        );
      }
    });
    invoke('get_config', { field: 'terminalWidth' }).then((value) => {
      if (typeof value === 'string') {
        this.settings.terminalWidth = +value;
        console.log(`Terminal width set to ${this.settings.terminalWidth}.`);
      }
    });
  }
}
