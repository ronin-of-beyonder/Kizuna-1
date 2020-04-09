import React, { useState } from "react";
import {
  IonContent,
  IonLabel,
  IonRow,
  IonCol,
  IonInput,
  IonButton,
} from "@ionic/react";
import styles from "./style.module.css";

const Register = ({
  history,
}: {
  history: { push: (param: string) => void };
}) => {
  const [lastName, setLastName] = useState("");
  const [firstName, setFirstName] = useState("");
  return (
    <IonContent>
      <div className={styles.Register}>
        <IonLabel>
          <h2>First Name</h2>
        </IonLabel>
        <IonRow
          className={`${styles.RegisterInput} ion-justify-content-center`}
        >
          <IonCol>
            <IonInput
              type="text"
              value={firstName}
              onIonChange={(e) =>
                setFirstName((e.target as HTMLInputElement).value)
              }
              placeholder="John"
            ></IonInput>
          </IonCol>
        </IonRow>
        <IonLabel className="ion-padding-top">
          <h2>Last Name</h2>
        </IonLabel>
        <IonRow
          className={`${styles.RegisterInput} ion-justify-content-center`}
        >
          <IonCol>
            <IonInput
              type="text"
              value={lastName}
              onIonChange={(e) =>
                setLastName((e.target as HTMLInputElement).value)
              }
              placeholder="Doe"
            ></IonInput>
          </IonCol>
        </IonRow>
        <IonButton onClick={() => {}} className={styles.Next}>
          Next
        </IonButton>
      </div>
    </IonContent>
  );
};

export default Register;
