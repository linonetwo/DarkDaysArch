import styled from 'styled-components';
import { useDispatch, useSelector } from 'react-redux';
import { TreeView, TreeItem } from '@material-ui/lab';
import { ExpandMore as ExpandMoreIcon, ChevronRight as ChevronRightIcon } from '@material-ui/icons';

import { Dispatch, RootState } from 'src/store/store';

const ItemListContainer = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
`;

const itemWidth = 50;
const ItemContainer = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  width: ${itemWidth}px;
  height: ${itemWidth}px;

  margin-bottom: ${itemWidth / 5}px;
`;

export function ItemList(): JSX.Element {
  const dispatch = useDispatch<Dispatch>();
  const terrain = useSelector((state: RootState) => state.knowledgeGraph.terrain);

  return (
    <ItemListContainer>
      {terrain.map((item) => (
        <ItemContainer key={item.id}>{item.id}</ItemContainer>
      ))}
    </ItemListContainer>
  );
}
