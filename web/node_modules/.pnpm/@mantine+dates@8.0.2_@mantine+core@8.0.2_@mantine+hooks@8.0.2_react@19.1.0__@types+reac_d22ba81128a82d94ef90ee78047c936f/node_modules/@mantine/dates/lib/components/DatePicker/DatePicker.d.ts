import { BoxProps, ElementProps, Factory, MantineComponentStaticProperties, StylesApiProps } from '@mantine/core';
import { CalendarLevel, DatePickerType, PickerBaseProps } from '../../types';
import { CalendarBaseProps, CalendarSettings, CalendarStylesNames } from '../Calendar';
import { DecadeLevelBaseSettings } from '../DecadeLevel';
import { MonthLevelBaseSettings } from '../MonthLevel';
import { YearLevelBaseSettings } from '../YearLevel';
export type DatePickerStylesNames = CalendarStylesNames;
export interface DatePickerBaseProps<Type extends DatePickerType = 'default'> extends PickerBaseProps<Type>, DecadeLevelBaseSettings, YearLevelBaseSettings, MonthLevelBaseSettings, CalendarBaseProps, Omit<CalendarSettings, 'hasNextLevel'> {
    /** Max level that user can go up to, `'decade'` by default */
    maxLevel?: CalendarLevel;
    /** Initial displayed level (uncontrolled) */
    defaultLevel?: CalendarLevel;
    /** Current displayed level (controlled) */
    level?: CalendarLevel;
    /** Called when level changes */
    onLevelChange?: (level: CalendarLevel) => void;
}
export interface DatePickerProps<Type extends DatePickerType = 'default'> extends BoxProps, DatePickerBaseProps<Type>, StylesApiProps<DatePickerFactory>, ElementProps<'div', 'onChange' | 'value' | 'defaultValue'> {
}
export type DatePickerFactory = Factory<{
    props: DatePickerProps;
    ref: HTMLDivElement;
    stylesNames: DatePickerStylesNames;
}>;
type DatePickerComponent = (<Type extends DatePickerType = 'default'>(props: DatePickerProps<Type> & {
    ref?: React.ForwardedRef<HTMLDivElement>;
}) => React.JSX.Element) & {
    displayName?: string;
} & MantineComponentStaticProperties<DatePickerFactory>;
export declare const DatePicker: DatePickerComponent;
export {};
