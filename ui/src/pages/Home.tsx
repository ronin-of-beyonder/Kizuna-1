import {IonContent,
  IonHeader,
  IonPage,
  IonTitle,
  IonToolbar,
  IonList,
  IonItem,
  IonLabel,
  IonCheckbox,
  IonNote,
  IonBadge,
  IonFab,
  IonFabButton,
  IonIcon,
  IonInput,
  IonButton
} from '@ionic/react';
import React, {useState} from 'react';
import {  pizza } from 'ionicons/icons';
import './Home.css';

const Home: React.FC = () => {
  const [text, setText] = useState('');

  return (
    <IonPage>
      <IonHeader>
        <IonToolbar>
          <IonTitle>Kizuna Connect</IonTitle>
        </IonToolbar>
      </IonHeader>
      <IonContent>
        <IonHeader collapse="condense">
          <IonToolbar>
            <IonTitle size="large">Kizuna Connect</IonTitle>
          </IonToolbar>
        </IonHeader>

        <IonList>
          <IonItem>
            <IonInput value={text} placeholder="Enter Input" onIonChange={e => setText(e.detail.value!)}></IonInput>
          </IonItem>
        </IonList>

        <IonButton size={'large'}>
          <IonIcon slot="start" icon={pizza} />
          Submit
        </IonButton>
      </IonContent>
    </IonPage>
  );
};

export default Home;
