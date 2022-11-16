export interface IStatBarSetting {
  show: boolean;
  background: string;
  foreground: string;
  emptyBackground: string;
}

export interface IPlayerInfoSettings {
  hp: IStatBarSetting;
  mana: IStatBarSetting;
  stamina: IStatBarSetting;
  stun: IStatBarSetting;
  focus: IStatBarSetting;
}
