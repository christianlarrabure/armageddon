import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { TerminalComponent } from './view/terminal/terminal.component';
import { NgTerminalModule } from 'ng-terminal';
import { InputComponent } from './components/input/input.component';
import { FormsModule } from '@angular/forms';
import { PlayerInfoComponent } from './components/player-info/player-info.component';
import { StatBarComponent } from './components/stat-bar/stat-bar.component';

@NgModule({
  declarations: [
    TerminalComponent,
    InputComponent,
    PlayerInfoComponent,
    StatBarComponent,
  ],
  imports: [CommonModule, NgTerminalModule, FormsModule],
  exports: [TerminalComponent],
})
export class TerminalModule {}
