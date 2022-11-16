import { Component, Input, OnChanges, SimpleChanges } from '@angular/core';
import { mapTo, Subject } from 'rxjs';
import { IStatBarSetting } from '../../interfaces/stat-bar.interface';
import {
  combineLatestWith,
  of,
  map,
  Observable,
  merge,
  tap,
  combineLatest,
} from 'rxjs';

@Component({
  selector: 'app-stat-bar',
  templateUrl: './stat-bar.component.html',
  styleUrls: ['./stat-bar.component.scss'],
})
export class StatBarComponent implements OnChanges {
  @Input() value?: String;
  @Input() maxValue?: String;
  @Input() color: 'red' | 'blue' | 'slate' | 'emerald' | 'violet' = 'red';
  @Input() settings?: IStatBarSetting;
  @Input() label?: string;

  value$ = new Subject<String | undefined>();
  maxValue$ = new Subject<String | undefined>();

  isLoaded$ = combineLatest([this.value$, this.maxValue$]).pipe(
    map((data) => {
      return data[0] !== undefined && data[1] !== undefined;
    })
  );

  ngOnChanges(changes: SimpleChanges) {
    if ('value' in changes) {
      this.value$.next(this.value);
    }
    if ('maxValue' in changes) {
      this.maxValue$.next(this.maxValue);
    }
  }

  getWidthString() {
    const value = +(this.value ?? 0);
    const maxValue = +(this.maxValue ?? 0);
    let percent = (value / maxValue) * 100;

    if (percent > 100) {
      percent = 100;
    } else if (percent < 0) {
      percent = 0;
    }

    const width = `${percent}%`;
    return width;
  }

  getBackground() {
    return `#${this.settings?.background ?? 'FFFFFFFF'}`;
  }

  getForeground() {
    return `#${this.settings?.foreground ?? '000000FF'}`;
  }

  getEmptyBackground() {
    return `#${this.settings?.emptyBackground ?? '000000FF'}`;
  }

  constructor() {}
}
