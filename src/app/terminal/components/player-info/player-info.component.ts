import { Component, AfterViewInit } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { TelnetService } from '../../../shared/services/telnet.service';
import { IPlayerInfoSettings } from '../../interfaces/stat-bar.interface';

@Component({
  selector: 'app-player-info',
  templateUrl: './player-info.component.html',
  styleUrls: ['./player-info.component.scss'],
})
export class PlayerInfoComponent implements AfterViewInit {
  constructor(private telnetService: TelnetService) {}
  prompt$ = this.telnetService.prompt$;
  settings?: IPlayerInfoSettings;

  ngAfterViewInit() {
    invoke<IPlayerInfoSettings>('get_config', { field: 'statBars' }).then(
      (value) => {
        this.settings = value;
      }
    );
  }
}
