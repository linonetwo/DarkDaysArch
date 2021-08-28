import React, { Component } from 'react';
import styled from 'styled-components';
import ReactDOM from 'react-dom';
import { connect, ConnectedProps } from 'react-redux';
import { search } from 'fast-fuzzy';
import { Menu, MenuItem, IconName } from '@blueprintjs/core';
import { RootState } from 'src/store/store';

export interface IMenuItem {
  icon?: IconName;
  title: string;
  type: string;
}

const menuHeight = 200;
interface MenuContainerProps {
  left?: string;
  opacity?: number;
  top?: string;
}
const MenuContainer = styled.div`
  position: absolute;
  top: ${({ top }: MenuContainerProps) => top ?? '-10000px'};
  left: ${({ left }: MenuContainerProps) => left ?? '-10000px'};
  z-index: 100;

  opacity: ${({ opacity }) => opacity ?? 0};
  transition: opacity 0.25s;

  ul.bp3-menu {
    max-height: ${menuHeight}px;
    overflow-y: auto;
  }
`;

// eslint-disable-next-line @typescript-eslint/explicit-function-return-type
function mapStoreToProps(store: RootState) {
  const { mouseX, mouseY } = store.cameraMouse;
  return {
    x: mouseX,
    y: mouseY,
  };
}
const connector = connect(mapStoreToProps);

type IPropsFromRedux = ConnectedProps<typeof connector>;
interface IProps extends IPropsFromRedux {
  items: IMenuItem[];
  mountPoint: string;
  open?: boolean;
}
interface IState {
  filter: string;
  items: IMenuItem[];
  position: number[];
}

class ContextMenu extends Component<IProps, IState> {
  state: IState = {
    filter: '',
    items: [],
    position: [0, 0],
  };

  static getDerivedStateFromProps(nextProps: IProps): Partial<IState> | null {
    const { x, y, items, open } = nextProps;
    // stop update menu position when it is open, so it won't follow cursor around
    if (open === true) {
      // eslint-disable-next-line unicorn/no-null
      return null;
    }
    return { position: [x, y], items: items };
  }

  shouldComponentUpdate(nextProps: IProps): boolean {
    // we need to update only if parent toggle this component, we get latest position data from parent
    if (this.props.open === nextProps.open) {
      return false;
    }
    return true;
  }

  componentDidMount(): void {
    document.addEventListener('keydown', this.handleSearch, false);
  }

  componentWillUnmount(): void {
    document.removeEventListener('keydown', this.handleSearch, false);
  }

  menuRef: HTMLDivElement | undefined;

  /**
   * When a mark button is clicked, toggle the current mark.
   */
  onClickMark = (item: IMenuItem, event: React.MouseEvent): void => {
    event.preventDefault();
    event.stopPropagation();
  };

  /**
   * Render a mark-toggling toolbar button.
   */
  renderMarkButton = (item: IMenuItem): JSX.Element => {
    const text = `${item.title}${item.type?.length > 0 ? ` ${item.type}` : ''}`;
    return <MenuItem key={text} text={text} icon={item.icon} onMouseDown={(event) => this.onClickMark(item, event)} />;
  };

  getMenuStyle = (): { left: string; opacity: number; top: string } => {
    if (this.menuRef === undefined || this.props.open !== true) return { opacity: 0, top: '-99999px', left: '-99999px' };
    return {
      opacity: 1,
      top: `${this.state.position[1]}px`,
      left: `${this.state.position[0]}px`,
    };
  };

  handleSearch = (event: KeyboardEvent): void => {
    const { opacity } = this.getMenuStyle();
    if (opacity > 0 && this.state.items.length > 0) {
      event.preventDefault();
      event.stopPropagation();
      if (event.key === 'Escape') {
        // 有时候用户按 Esc 其实会希望关闭选单
        if (this.state.filter !== '') {
          // 如果有字在过滤器里，才会是指清空过滤器
          this.setState({ filter: '' });
        }
      } else if (event.key === 'Backspace' || event.key === 'Delete') {
        this.setState(({ filter }) => ({
          filter: filter.slice(0, Math.max(0, filter.length - 1)),
        }));
      } else if (/^[\da-z]$/.test(event.key)) {
        this.setState(({ filter }) => ({ filter: filter + event.key }));
      }
    }
  };

  render(): JSX.Element {
    const { opacity, top, left } = this.getMenuStyle();
    const mountPoint = document.querySelector(`#${this.props.mountPoint}`);
    if (mountPoint !== null && mountPoint !== undefined) {
      const itemsToDisplay =
        this.state.filter.length > 0
          ? search(this.state.filter, this.state.items, {
              keySelector: (item: IMenuItem) => item.type,
            })
          : this.state.items;
      return ReactDOM.createPortal(
        <MenuContainer
          data-usage="context-menu"
          opacity={opacity}
          top={top}
          left={left}
          ref={(element) => {
            if (element !== null && element !== undefined) {
              this.menuRef = element;
            }
          }}>
          <Menu>{itemsToDisplay.map((item) => this.renderMarkButton(item))}</Menu>
          {this.state.filter}
          {this.state.items.length === 0 && 'No Data'}
        </MenuContainer>,
        mountPoint,
      );
    }
    return <div />;
  }
}

export default connector(ContextMenu);
